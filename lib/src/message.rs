use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Test,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Test,
    Disconnect,
}
