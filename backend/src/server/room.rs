use std::{collections::HashMap, sync::Arc};

use lib::{
    figure::FigureData,
    message::{RequestType, ResponseType, ServerMessage},
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use tracing::log;

use super::{user::User, ServerAppMessage};

#[derive(Debug)]
pub enum RoomMessage {
    LeaveUser(Arc<str>),
    AddFigure(FigureData),
    RequestInfo(Arc<str>, RequestType),
}

pub struct Room {
    id: Arc<str>,
    server_app_sender: Sender<ServerAppMessage>,
    users: Arc<Mutex<HashMap<Arc<str>, User>>>,
    figures: Arc<Mutex<Vec<FigureData>>>,
    sender: Sender<RoomMessage>, //Pass to new_user so that room's receiver can receive a message from user.
}

impl Room {
    pub fn new(id: Arc<str>, server_app_sender: Sender<ServerAppMessage>) -> Self {
        let (sender, receiver) = mpsc::channel(1000);

        let room = Self {
            id,
            server_app_sender,
            users: Arc::new(Mutex::new(HashMap::new())),
            figures: Arc::new(Mutex::new(Vec::new())),
            sender,
        };

        room.run(receiver);

        room
    }

    #[allow(clippy::single_match)]
    fn run(&self, mut receiver: Receiver<RoomMessage>) {
        let users_clone = self.users.clone();
        let server_app_sender_clone = self.server_app_sender.clone();
        let figures_clone = self.figures.clone();
        let room_id = self.id.clone();
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
                        figures_clone.lock().await.push(data.clone());
                        broadcast(users_clone.clone(), ServerMessage::FigureAdded(data)).await;
                    }
                    RoomMessage::RequestInfo(user_id, request_type) => match request_type {
                        RequestType::CurrentFigures => {
                            let mut users_lock = users_clone.lock().await;
                            let vec = figures_clone.lock().await.clone();
                            if let Some(user) = users_lock.get_mut(&user_id) {
                                user.send_message(ServerMessage::ResponseInfo(
                                    ResponseType::CurrentFigures(vec),
                                ))
                                .await;
                            }
                        }
                        _ => {}
                    },
                }
            }
        });
    }

    pub async fn join_user(&self, mut new_user: User) {
        let new_user_id = new_user.id();
        new_user.set_channel(self.sender.clone()).await;

        {
            let mut users_lock = self.users.lock().await;
            users_lock.insert(new_user.id(), new_user);
        }

        broadcast(
            self.users.clone(),
            ServerMessage::UserJoined(new_user_id.to_string()),
        )
        .await;
    }

    pub async fn check_exist_user(&self, user_id: &str) -> bool {
        self.users.lock().await.get(user_id).is_some()
    }
}

async fn broadcast(users: Arc<Mutex<HashMap<Arc<str>, User>>>, message: ServerMessage) {
    let mut users_lock = users.lock().await;

    for (_, user) in users_lock.iter_mut() {
        user.send_message(message.clone()).await;
    }
}
