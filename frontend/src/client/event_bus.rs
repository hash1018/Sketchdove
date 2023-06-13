use lib::message::ServerMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{HandlerId, Public, Worker, WorkerLink};
#[derive(Serialize, Deserialize, Debug)]
pub struct EventBusMessage {
    pub message: ServerMessage,
}

pub struct EventBus {
    link: WorkerLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Worker for EventBus {
    type Input = EventBusMessage;
    type Message = ();
    type Output = ServerMessage;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        let msg = msg.message;
        for subscriber in self.subscribers.iter() {
            self.link.respond(*subscriber, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        log::debug!("connected");
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        log::debug!("disconnected");
        self.subscribers.remove(&id);
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }

    fn resource_path_is_relative() -> bool {
        true
    }
}
