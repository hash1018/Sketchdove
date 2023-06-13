use frontend::client::event_bus::EventBus;
use yew_agent::PublicWorker;

fn main() {
    EventBus::register();
}
