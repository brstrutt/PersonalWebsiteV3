use std::{cell::RefCell, rc::Rc};
use leptos::leptos_dom;
use wasmfenbein3d::core::state::GameState;
use leptos::web_sys::{HtmlCanvasElement, EventTarget};
use leptos::ev::{
    self, Event, KeyboardEvent, MouseEvent, TouchEvent
};
use leptos::prelude::*;
use crate::app::utils::add_event_listener_with_callback;


pub fn setup_controls(canvas_element: HtmlCanvasElement, state: Rc<RefCell<GameState>>) {
    setup_keyboard_and_mouse_controls(canvas_element.clone(), state.clone());
    setup_camera_touch_control(canvas_element, state);
}

fn setup_keyboard_and_mouse_controls(canvas_element: HtmlCanvasElement, state: Rc<RefCell<GameState>>) {
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

fn setup_camera_touch_control(canvas_element: HtmlCanvasElement, state: Rc<RefCell<GameState>>) {
    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(canvas_element.clone()), "touchstart", move |e: TouchEvent| {
        e.prevent_default();
        let mut state = cloned_state.borrow_mut();

        let touch_points = e.target_touches();
        if touch_points.length() > 0 {
            let touch_x_position = touch_points
                .item(0)
                .expect("Failed to get first touch point on the canvas")
                .screen_x();
            state.input.last_canvas_touch_point_x = Some(touch_x_position);
        }
    });

    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(canvas_element.clone()), "touchmove", move |e: TouchEvent| {
        e.prevent_default();
        let mut state = cloned_state.borrow_mut();
        const ACCELERATION: i32 = 4;

        let touch_points = e.target_touches();
        if touch_points.length() > 0 {
            let touch_x_position = touch_points
                .item(0)
                .expect("Failed to get first touch point on the canvas")
                .screen_x();

            if state.input.last_canvas_touch_point_x.is_some() {
                state.input.camera_rotation = (state.input.last_canvas_touch_point_x.unwrap()
                    - touch_x_position)
                    * ACCELERATION;
            }

            state.input.last_canvas_touch_point_x = Some(touch_x_position);
            state.input.touch_has_moved_camera = true;
        }
    });

    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(canvas_element), "touchend", move |e: TouchEvent| {
        e.prevent_default();
        let mut state = cloned_state.borrow_mut();
        state.input.last_canvas_touch_point_x = None;
    });
}