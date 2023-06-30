use std::{collections::HashMap, sync::Arc};

use lib::message::ServerMessage;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use tracing::log;

use super::{user::User, ServerAppMessage};

#[derive(Debug)]
pub enum RoomMessage {
    LeaveUser(String),
}

pub struct Room {
    id: String,
    server_app_sender: Sender<ServerAppMessage>,
    users: Arc<Mutex<HashMap<String, User>>>,
    sender: Sender<RoomMessage>,
}

impl Room {
    pub fn new(id: String, server_app_sender: Sender<ServerAppMessage>) -> Self {
        let (sender, receiver) = mpsc::channel(1000);

        let room = Self {
            id,
            server_app_sender,
            users: Arc::new(Mutex::new(HashMap::new())),
            sender,
        };

        room.set_channel(receiver);

        room
    }

    fn set_channel(&self, mut receiver: Receiver<RoomMessage>) {
        let users_clone = self.users.clone();
        let server_app_sender_clone = self.server_app_sender.clone();
        let room_id = self.id.clone();
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                match message {
                    RoomMessage::LeaveUser(user_id) => {
                        log::info!("LeaveUser user_id = {user_id}");
                        let mut users_write = users_clone.lock().await;
                        users_write.remove(&user_id);
                        log::info!("now users = {0:?}", *users_write);
                        if users_write.is_empty() {
                            let _ = server_app_sender_clone
                                .send(ServerAppMessage::DeleteRoom(room_id.clone()))
                                .await;
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn join_user(&self, mut new_user: User) {
        new_user.set_sender(self.sender.clone()).await;

        let mut users_lock = self.users.lock().await;

        for (_, user) in users_lock.iter_mut() {
            user.send_message(ServerMessage::UserJoined(new_user.id()))
                .await;
        }
        users_lock.insert(new_user.id(), new_user);

        log::info!("now users = {0:?}", *users_lock);
    }
}
