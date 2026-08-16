use leptos::{
    wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast},
    web_sys::EventTarget,
};

pub fn add_event_listener_with_callback<E: FromWasmAbi, T: FnMut(E)>(
    object: EventTarget,
    event_name: &str,
    mut run: T,
) {
    let callback = Closure::wrap(Box::new(move |e: E| {
        run(e);
    }) as Box<dyn FnMut(_)>);
    object
        .add_event_listener_with_callback(event_name, callback.as_ref().unchecked_ref())
        .expect("Failed to setup event listener with custom rust lambda");
    callback.forget();
}
