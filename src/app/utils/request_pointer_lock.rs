use web_sys::{wasm_bindgen::JsCast, Event, HtmlElement};

pub fn request_pointer_lock(event: Event) {
    event
        .target()
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .request_pointer_lock();
}
