use std::{cell::RefCell, rc::Rc};
use leptos::leptos_dom;
use wasmfenbein3d::core::state::GameState;
use leptos::web_sys::{HtmlCanvasElement, EventTarget};
use leptos::ev::{
    self,
    Event,
    KeyboardEvent,
    MouseEvent
};
use leptos::prelude::*;
use crate::app::utils::add_event_listener_with_callback;


pub fn setup_controls(canvas_element: HtmlCanvasElement, state: Rc<RefCell<GameState>>) {
    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(document()), "pointerlockchange", move |_: Event| {
        let mut state = cloned_state.borrow_mut();
        state.input.pointer_locked = document().pointer_lock_element().is_some();
        if !state.input.pointer_locked {
            state.input.sprint = false;
            state.input.move_left = false;
            state.input.move_right = false;
            state.input.move_forward = false;
            state.input.move_backward = false;
        }
    });
    let cloned_state = state.clone();
    leptos_dom::helpers::window_event_listener(ev::mousemove, move |e: MouseEvent| {
        let mut state = cloned_state.borrow_mut();

        if state.input.pointer_locked {
            state.input.camera_rotation += e.movement_x();
        }
    });
    let cloned_canvas = canvas_element.clone();
    add_event_listener_with_callback(EventTarget::from(cloned_canvas.clone()), "click", move |_: Event| {
        cloned_canvas.request_pointer_lock();
    });


    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(document()), "keydown", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
        if state.input.pointer_locked {
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = true,
                "d" | "D" => state.input.move_right = true,
                "w" | "W" => state.input.move_forward = true,
                "s" | "S" => state.input.move_backward = true,
                &_ => return,
            }
        }
    });

    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(document()), "keyup", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
        if state.input.pointer_locked {
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = false,
                "d" | "D" => state.input.move_right = false,
                "w" | "W" => state.input.move_forward = false,
                "s" | "S" => state.input.move_backward = false,
                &_ => return,
            }
        }
    });
}