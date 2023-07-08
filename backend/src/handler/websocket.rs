use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::StreamExt;
use lib::message::ClientMessage;
use tracing::log;

use crate::server::{user::User, ServerApp};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(server_app): State<Arc<ServerApp>>,
) -> impl IntoResponse {
    log::info!("websocket connected");
    ws.on_upgrade(|socket| websocket(socket, server_app))
}

async fn websocket(stream: WebSocket, server_app: Arc<ServerApp>) {
    let (sender, mut receiver) = stream.split();

    let mut room_id = None;
    let mut user_id = None;
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(message) = message {
            let message: ClientMessage = serde_json::from_str(&message).unwrap();
            match message {
                ClientMessage::Join(room_id_inner, user_id_inner) => {
                    let user_id_inner: Arc<str> = Arc::from(user_id_inner);
                    if !server_app.check_exist_room(&room_id_inner).await {
                        let room_id_inner: Arc<str> = Arc::from(room_id_inner);
                        if server_app.make_room(room_id_inner.clone()).await.is_ok() {
                            room_id = Some(room_id_inner);
                            user_id = Some(user_id_inner);
                            break;
                        } else {
                            return;
                        }
                    } else {
                        room_id = Some(Arc::from(room_id_inner));
                        user_id = Some(user_id_inner);
                        break;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    if let (Some(user_id), Some(room_id)) = (user_id, room_id) {
        let user = User::new(user_id, sender, receiver);
        let _ = server_app.join_room(room_id, user).await;
    }
}
