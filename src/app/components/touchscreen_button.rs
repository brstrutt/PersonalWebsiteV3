use crate::app::utils::add_event_listener_with_callback;
use leptos::{
    ev::Event,
    html::Button,
    prelude::*,
    web_sys::{EventTarget, HtmlButtonElement},
};

#[component]
pub fn touchscreen_button(
    /// Mutable reference to allow reference to controls to bubble up
    #[prop(into)]
    node_ref: NodeRef<Button>,
    children: Children,
) -> impl IntoView {
    Effect::new(move |_| {
        if let Some(button_ref) = node_ref.get() {
            setup_touchscreen_button_behaviour(button_ref)
        }
    });

    view! { <button node_ref=node_ref>{children()}</button> }
}

/// Function to update the behaviour of an array of buttons to work better on touch devices
/// Specifically prevents normal long press behaviour to allow holding
/// the buttons down without accidentally opening a context menu or highlighting the button text
fn setup_touchscreen_button_behaviour(button: HtmlButtonElement) {
    let cloned_button = button.clone();
    let target = EventTarget::from(button.clone());
    add_event_listener_with_callback(target.clone(), "mousedown", move |e: Event| {
        e.prevent_default();
        set_active(true, cloned_button.clone());
    });
    let cloned_button = button.clone();
    add_event_listener_with_callback(target.clone(), "touchstart", move |e: Event| {
        e.prevent_default();
        set_active(true, cloned_button.clone());
    });

    let cloned_button = button.clone();
    add_event_listener_with_callback(target.clone(), "mouseup", move |e: Event| {
        e.prevent_default();
        set_active(false, cloned_button.clone());
    });
    add_event_listener_with_callback(target.clone(), "touchend", move |e: Event| {
        e.prevent_default();
        set_active(false, button.clone());
    });
}

fn set_active(active: bool, button: HtmlButtonElement) {
    if active {
        button.set_class_name("active")
    } else {
        button.set_class_name("")
    }
}
