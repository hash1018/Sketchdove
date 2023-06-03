use frontend::pages::main_app;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<main_app::Main>::new().render();
}
