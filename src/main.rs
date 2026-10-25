use leptos::mount::mount_to_body;
mod app;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    mount_to_body(app::App);
}
