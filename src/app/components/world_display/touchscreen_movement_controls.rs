use crate::app::utils::add_event_listener_with_callback;
use leptos::{
    ev::Event,
    web_sys::{EventTarget, HtmlButtonElement},
};

/// Function to update the behaviour of an array of buttons to work better on touch devices
/// Specifically prevents normal long press behaviour to allow holding
/// the buttons down without accidentally opening a context menu or highlighting the button text
pub fn setup_touchscreen_button_behaviour(button: &HtmlButtonElement) {
    let target = EventTarget::from(button.clone());
    add_event_listener_with_callback(target.clone(), "mousedown", on_button_touched(&button));
    add_event_listener_with_callback(target.clone(), "touchstart", on_button_touched(&button));

    add_event_listener_with_callback(target.clone(), "mouseup", on_button_untouched(&button));
    add_event_listener_with_callback(target.clone(), "touchend", on_button_untouched(&button));
}

fn on_button_touched<'button>(button: &'button HtmlButtonElement) -> Box<dyn Fn(Event) + 'button> {
    on_button_touch_event(button, EventType::Start)
}

fn on_button_untouched<'button>(
    button: &'button HtmlButtonElement,
) -> Box<dyn Fn(Event) + 'button> {
    on_button_touch_event(button, EventType::Stop)
}

#[derive(PartialEq)]
enum EventType {
    Start,
    Stop,
}

fn on_button_touch_event<'button>(
    button: &'button HtmlButtonElement,
    event_type: EventType,
) -> Box<dyn Fn(Event) + 'button> {
    Box::new(move |e: Event| {
        e.prevent_default();
        set_active(event_type == EventType::Start, button);
    })
}

fn set_active(active: bool, button: &HtmlButtonElement) {
    if active {
        button.set_class_name("active")
    } else {
        button.set_class_name("")
    }
}
