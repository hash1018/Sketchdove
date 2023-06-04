use std::cell::RefCell;

use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_utils::errors::JsError;
use lib::message::{ClientMessage, ServerMessage};
use reqwasm::websocket::{futures::WebSocket, Message};

use wasm_bindgen_futures::spawn_local;
use yew_agent::Dispatched;

use super::event_bus::{EventBus, EventBusMessage};
use lib::{IP_ADDRESS, PORT};

#[derive(Debug)]
pub enum WebSocketError {
    OpenError(JsError),
}

#[derive(Clone, Default)]
pub struct WebsocketService {
    tx: RefCell<Option<Sender<ClientMessage>>>,
}

impl WebsocketService {
    pub fn new() -> Self {
        Self {
            tx: RefCell::new(None),
        }
    }

    pub fn connect(&self) -> Result<(), WebSocketError> {
        let address = format!("ws:[{IP_ADDRESS}]:{PORT}/websocket");

        let ws = match WebSocket::open(&address) {
            Ok(ws) => ws,
            Err(error) => {
                return Err(WebSocketError::OpenError(error));
            }
        };

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<ClientMessage>(1000);

        let mut event_bus = EventBus::dispatcher();

        spawn_local(async move {
            while let Some(message) = in_rx.next().await {
                let s = serde_json::to_string(&message).unwrap();
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(message)) => {
                        let message: ServerMessage = serde_json::from_str(&message).unwrap();
                        event_bus.send(EventBusMessage { message });
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(_val) = decoded {
                            //TODO:
                        }
                    }
                    Err(_e) => {}
                }
            }
        });

        *self.tx.borrow_mut() = Some(in_tx);
        Ok(())
    }

    pub fn disconnect(&self) {
        let mut sender = self.tx.borrow_mut().take().unwrap();
        let _ = sender.try_send(ClientMessage::Disconnect);
    }

    pub fn send(&self, message: ClientMessage) -> bool {
        let sender = self.tx.borrow_mut().take();
        if sender.is_none() {
            return false;
        }

        let mut sender = sender.unwrap();
        let _ = sender.try_send(message);

        *self.tx.borrow_mut() = Some(sender);

        true
    }
}
