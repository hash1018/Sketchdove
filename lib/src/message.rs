use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    UserJoined(UserId),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Disconnect,
    Join(RoomId, UserId),
}

type RoomId = String;
type UserId = String;
