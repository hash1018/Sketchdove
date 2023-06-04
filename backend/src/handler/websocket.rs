use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use lib::message::{ClientMessage, ServerMessage};
use tracing::log;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    log::info!("connected");
    ws.on_upgrade(websocket)
}

async fn websocket(stream: WebSocket) {
    let (mut sender, mut receiver) = stream.split();

    /*
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    */

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(message) = message {
                log::info!("text message");
                let message: ClientMessage = serde_json::from_str(&message).unwrap();
                match message {
                    ClientMessage::Test => {
                        log::info!("received message from client");
                        let server_message = serde_json::to_string(&ServerMessage::Test).unwrap();
                        let _ = sender.send(Message::Text(server_message)).await;
                    }
                    ClientMessage::Disconnect => {
                        break;
                    }
                }
            } else {
                log::info!("other message");
            }
        }
    });

    tokio::select! {
        _ = (&mut recv_task) => {}
    };
    log::info!("closed");
}
