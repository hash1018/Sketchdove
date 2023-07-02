use std::{fmt, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use lib::message::{ClientMessage, ServerMessage};
use tokio::sync::{mpsc::Sender, Mutex};
use tracing::log::{self};

use super::room::RoomMessage;

pub struct User {
    id: String,
    room_sender: Arc<Mutex<Option<Sender<RoomMessage>>>>,
    socket_sender: Option<SplitSink<WebSocket, Message>>,
    socket_receiver: Option<SplitStream<WebSocket>>,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User").field("id", &self.id).finish()
    }
}

impl User {
    pub fn new(
        id: String,
        sender: SplitSink<WebSocket, Message>,
        receiver: SplitStream<WebSocket>,
    ) -> Self {
        Self {
            id,
            room_sender: Arc::new(Mutex::new(None)),
            socket_sender: Some(sender),
            socket_receiver: Some(receiver),
        }
    }

    pub async fn set_channel(
        &mut self,
        room_sender: Sender<RoomMessage>,
        room_receiver: tokio::sync::broadcast::Receiver<ServerMessage>,
    ) {
        let id = self.id.clone();

        *self.room_sender.lock().await = Some(room_sender.clone());
        let room_sender_clone = self.room_sender.clone();

        let socket_sender = self.socket_sender.take().unwrap();
        let socket_receiver = self.socket_receiver.take().unwrap();

        tokio::spawn(async move {
            handle_message(
                id,
                room_sender_clone,
                room_receiver,
                socket_sender,
                socket_receiver,
            )
            .await;
        });
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}

async fn handle_message(
    id: String,
    room_sender: Arc<Mutex<Option<Sender<RoomMessage>>>>,
    mut room_receiver: tokio::sync::broadcast::Receiver<ServerMessage>,
    mut socket_sender: SplitSink<WebSocket, Message>,
    mut socket_receiver: SplitStream<WebSocket>,
) {
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = socket_receiver.next().await {
            if let Message::Text(message) = message {
                log::info!("text message");
                let message: ClientMessage = serde_json::from_str(&message).unwrap();
                let room_message = match message {
                    ClientMessage::Disconnect => {
                        let sender_lock = room_sender.lock().await;
                        if let Some(sender) = &*sender_lock {
                            sender.send(RoomMessage::LeaveUser(id)).await.unwrap();
                        }
                        break;
                    }
                    ClientMessage::AddFigure(data) => RoomMessage::AddFigure(data),
                    _ => {
                        continue;
                    }
                };

                let sender_lock = room_sender.lock().await;
                if let Some(sender) = &*sender_lock {
                    sender.send(room_message).await.unwrap();
                }
            } else if let Message::Close(_) = message {
                let sender_lock = room_sender.lock().await;
                if let Some(sender) = &*sender_lock {
                    sender.send(RoomMessage::LeaveUser(id)).await.unwrap();
                }
                break;
            } else {
                log::info!("other message {message:?}");
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(message) = room_receiver.recv().await {
            let message = serde_json::to_string(&message).unwrap();
            let _ = socket_sender.send(Message::Text(message)).await;
        }
    });

    tokio::select! {
        _ = (&mut recv_task) => send_task.abort(),
        _ = (&mut send_task) => recv_task.abort(),
    };
}
