use serde::{Deserialize, Serialize};

use crate::figure::FigureData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    UserJoined(UserId),
    FigureAdded(FigureData),
    ResponseInfo(ResponseType),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Disconnect,
    Join(RoomId, UserId),
    AddFigure(FigureData),
    RequestInfo(RequestType),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RequestType {
    CurrentFigures,
    CheckRoomExist(RoomId),
    CheckUserExist(RoomId, UserId),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ResponseType {
    CurrentFigures(Vec<FigureData>),
    ResponseRoomExist(bool),
    ResponseUserExist(Option<bool>),
    InvalidRequest(RequestType),
}

pub type RoomId = String;
pub type UserId = String;
