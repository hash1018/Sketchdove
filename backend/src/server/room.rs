use std::{collections::HashMap, sync::Arc};

use lib::{figure::FigureData, message::ServerMessage};
use tokio::sync::{
    broadcast,
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use tracing::log;

use super::{user::User, ServerAppMessage};

#[derive(Debug)]
pub enum RoomMessage {
    LeaveUser(String),
    AddFigure(FigureData),
}

pub struct Room {
    id: String,
    server_app_sender: Sender<ServerAppMessage>,
    users: Arc<Mutex<HashMap<String, User>>>,
    figures: Arc<Mutex<Vec<FigureData>>>,
    sender: Sender<RoomMessage>, //To pass to new_user so that room receiver can receive a message from user.
    user_sender: tokio::sync::broadcast::Sender<ServerMessage>,
}

impl Room {
    pub fn new(id: String, server_app_sender: Sender<ServerAppMessage>) -> Self {
        let (sender, receiver) = mpsc::channel(1000);
        let (user_sender, _) = broadcast::channel(1000);

        let room = Self {
            id,
            server_app_sender,
            users: Arc::new(Mutex::new(HashMap::new())),
            figures: Arc::new(Mutex::new(Vec::new())),
            sender,
            user_sender,
        };

        room.run(receiver);

        room
    }

    fn run(&self, mut receiver: Receiver<RoomMessage>) {
        let users_clone = self.users.clone();
        let server_app_sender_clone = self.server_app_sender.clone();
        let figures_clone = self.figures.clone();
        let room_id = self.id.clone();
        let user_sender = self.user_sender.clone();
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                match message {
                    RoomMessage::LeaveUser(user_id) => {
                        log::info!("LeaveUser user_id = {user_id}");
                        let mut users_lock = users_clone.lock().await;
                        users_lock.remove(&user_id);
                        log::info!("now users = {0:?}", *users_lock);
                        if users_lock.is_empty() {
                            let _ = server_app_sender_clone
                                .send(ServerAppMessage::DeleteRoom(room_id.clone()))
                                .await;
                            break;
                        }
                    }
                    RoomMessage::AddFigure(data) => {
                        log::info!("Add Figure {data:?}");
                        figures_clone.lock().await.push(data.clone());
                        let _ = user_sender.send(ServerMessage::FigureAdded(data));
                    }
                }
            }
        });
    }

    pub async fn join_user(&self, mut new_user: User) {
        let _ = self
            .user_sender
            .send(ServerMessage::UserJoined(new_user.id()));

        let receiver = self.user_sender.subscribe();
        new_user.set_channel(self.sender.clone(), receiver).await;

        let mut users_lock = self.users.lock().await;
        users_lock.insert(new_user.id(), new_user);

        log::info!("now users = {0:?}", *users_lock);
    }
}
