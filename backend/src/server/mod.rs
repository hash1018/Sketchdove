use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use tracing::log;

use self::{room::Room, user::User};

pub mod room;
pub mod user;

#[derive(Debug)]
pub enum ServerAppMessage {
    DeleteRoom(Arc<str>),
}

#[derive(Debug)]
pub enum ServerAppError {
    RoomAlreadyExist(Arc<str>),
    RoomDoesNotExist(Arc<str>),
}

pub struct ServerApp {
    rooms: Arc<Mutex<HashMap<Arc<str>, Room>>>,
    sender: Sender<ServerAppMessage>,
}

impl ServerApp {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1000);
        let app = ServerApp {
            rooms: Arc::new(Mutex::new(HashMap::new())),
            sender,
        };

        app.run(receiver);

        app
    }

    fn run(&self, mut receiver: Receiver<ServerAppMessage>) {
        let rooms_clone = self.rooms.clone();
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                match message {
                    ServerAppMessage::DeleteRoom(id) => {
                        log::info!("Delete Room room_id = {id}");
                        rooms_clone.lock().await.remove(&id);
                    }
                }
            }
        });
    }

    pub async fn make_room(&self, room_id: Arc<str>) -> Result<(), ServerAppError> {
        log::info!("make room room_id = {room_id}");
        let mut rooms_write = self.rooms.lock().await;
        if rooms_write.get(&*room_id).is_some() {
            return Err(ServerAppError::RoomAlreadyExist(room_id));
        }

        let new_room = Room::new(room_id.clone(), self.sender.clone());

        rooms_write.insert(room_id, new_room);

        Ok(())
    }

    pub async fn check_exist_room(&self, room_id: &str) -> bool {
        self.rooms.lock().await.get(room_id).is_some()
    }

    pub async fn check_exist_user(
        &self,
        room_id: &str,
        user_id: &str,
    ) -> Result<bool, ServerAppError> {
        if let Some(room) = self.rooms.lock().await.get(room_id) {
            return Ok(room.check_exist_user(user_id).await);
        }

        Err(ServerAppError::RoomDoesNotExist(room_id.into()))
    }

    pub async fn join_room(&self, room_id: Arc<str>, user: User) -> Result<(), ServerAppError> {
        log::info!("join_room room_id = {room_id}");
        let mut rooms_write = self.rooms.lock().await;

        let room = rooms_write.get_mut(&*room_id);

        if let Some(room) = room {
            room.join_user(user).await;
        } else {
            return Err(ServerAppError::RoomDoesNotExist(room_id));
        }

        Ok(())
    }
}
