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
    sender: SplitSink<WebSocket, Message>,
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
        let user = Self {
            id: id.clone(),
            room_sender: Arc::new(Mutex::new(None)),
            sender,
        };

        let room_sender_clone = user.room_sender.clone();
        tokio::spawn(async move {
            handle_client_message(id, room_sender_clone, receiver).await;
        });

        user
    }

    pub async fn set_sender(&mut self, sender: Sender<RoomMessage>) {
        *self.room_sender.lock().await = Some(sender);
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub async fn send_message(&mut self, message: ServerMessage) {
        let message = serde_json::to_string(&message).unwrap();
        let _ = self.sender.send(Message::Text(message)).await;
    }
}

async fn handle_client_message(
    id: String,
    sender: Arc<Mutex<Option<Sender<RoomMessage>>>>,
    mut receiver: SplitStream<WebSocket>,
) {
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(message) = message {
                log::info!("text message");
                let message: ClientMessage = serde_json::from_str(&message).unwrap();
                if message == ClientMessage::Disconnect {
                    let sender_lock = sender.lock().await;
                    if let Some(sender) = &*sender_lock {
                        sender.send(RoomMessage::LeaveUser(id)).await.unwrap();
                    }
                    break;
                }
            } else if let Message::Close(_) = message {
                let sender_lock = sender.lock().await;
                if let Some(sender) = &*sender_lock {
                    sender.send(RoomMessage::LeaveUser(id)).await.unwrap();
                }
                break;
            } else {
                log::info!("other message {message:?}");
            }
        }
    });

    tokio::select! {
        _ = (&mut recv_task) => {}
    };
}
