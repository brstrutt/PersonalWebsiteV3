use crate::app::utils::add_event_listener_with_callback;
use leptos::ev::{self, Event, KeyboardEvent, MouseEvent, TouchEvent};
use leptos::leptos_dom;
use leptos::prelude::*;
use leptos::web_sys::{EventTarget, HtmlButtonElement, HtmlCanvasElement};
use std::{cell::RefCell, rc::Rc};
use wasmfenbein3d::core::state::GameState;

mod movement;
use movement::*;

pub fn setup_controls(
    canvas_element: HtmlCanvasElement,
    state: Rc<RefCell<GameState>>,
    left_button_ref: HtmlButtonElement,
    right_button_ref: HtmlButtonElement,
    up_button_ref: HtmlButtonElement,
    down_button_ref: HtmlButtonElement,
) {
    setup_keyboard_and_mouse_controls(canvas_element.clone(), state.clone());
    setup_camera_touch_control(canvas_element, state.clone());
    setup_touchscreen_button_behaviour(&[
        &left_button_ref,
        &right_button_ref,
        &up_button_ref,
        &down_button_ref,
    ]);
    setup_touchscreen_movement_controls(
        state,
        left_button_ref,
        right_button_ref,
        up_button_ref,
        down_button_ref,
    );
}

fn setup_keyboard_and_mouse_controls(
    canvas_element: HtmlCanvasElement,
    state: Rc<RefCell<GameState>>,
) {
    let cloned_state = state.clone();
    add_event_listener_with_callback(
        EventTarget::from(document()),
        "pointerlockchange",
        move |_: Event| {
            let mut state = cloned_state.borrow_mut();
            state.input.pointer_locked = document().pointer_lock_element().is_some();
            if !state.input.pointer_locked {
                reset_movement(&mut state)
            }
        },
    );
    let cloned_state = state.clone();
    leptos_dom::helpers::window_event_listener(ev::mousemove, move |e: MouseEvent| {
        let mut state = cloned_state.borrow_mut();

        if state.input.pointer_locked {
            state.input.camera_rotation += e.movement_x();
        }
    });
    let cloned_canvas = canvas_element.clone();
    add_event_listener_with_callback(
        EventTarget::from(cloned_canvas.clone()),
        "click",
        move |_: Event| {
            cloned_canvas.request_pointer_lock();
        },
    );

    let cloned_state = state.clone();
    add_event_listener_with_callback(
        EventTarget::from(document()),
        "keydown",
        move |e: KeyboardEvent| {
            let mut state = cloned_state.borrow_mut();
            if state.input.pointer_locked {
                state.input.sprint = e.shift_key();
                if let Some(direction) = key_to_direction(e.key().as_str()) {
                    start_move(&mut state, &direction);
                }
            }
        },
    );

    let cloned_state = state.clone();
    add_event_listener_with_callback(
        EventTarget::from(document()),
        "keyup",
        move |e: KeyboardEvent| {
            let mut state = cloned_state.borrow_mut();
            if state.input.pointer_locked {
                state.input.sprint = e.shift_key();
                if let Some(direction) = key_to_direction(e.key().as_str()) {
                    stop_move(&mut state, &direction);
                }
            }
        },
    );
}

fn setup_camera_touch_control(canvas_element: HtmlCanvasElement, state: Rc<RefCell<GameState>>) {
    let cloned_state = state.clone();
    add_event_listener_with_callback(
        EventTarget::from(canvas_element.clone()),
        "touchstart",
        move |e: TouchEvent| {
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
        },
    );

    let cloned_state = state.clone();
    add_event_listener_with_callback(
        EventTarget::from(canvas_element.clone()),
        "touchmove",
        move |e: TouchEvent| {
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
        },
    );

    let cloned_state = state.clone();
    add_event_listener_with_callback(
        EventTarget::from(canvas_element),
        "touchend",
        move |e: TouchEvent| {
            e.prevent_default();
            let mut state = cloned_state.borrow_mut();
            state.input.last_canvas_touch_point_x = None;
        },
    );
}

fn setup_touchscreen_button_behaviour(buttons: &[&HtmlButtonElement]) {
    for button in buttons {
        let target = EventTarget::from((*button).clone());
        add_event_listener_with_callback(target.clone(), "mousedown", on_button_touched(&button));
        add_event_listener_with_callback(target.clone(), "touchstart", on_button_touched(&button));

        add_event_listener_with_callback(target.clone(), "mouseup", on_button_untouched(&button));
        add_event_listener_with_callback(target.clone(), "touchend", on_button_untouched(&button));
    }
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

fn setup_touchscreen_movement_controls(
    state: Rc<RefCell<GameState>>,
    left_button_ref: HtmlButtonElement,
    right_button_ref: HtmlButtonElement,
    up_button_ref: HtmlButtonElement,
    down_button_ref: HtmlButtonElement,
) {
    setup_movement_button(state.clone(), up_button_ref, Direction::Forward);
    setup_movement_button(state.clone(), down_button_ref, Direction::Backward);
    setup_movement_button(state.clone(), left_button_ref, Direction::Left);
    setup_movement_button(state.clone(), right_button_ref, Direction::Right);
}

fn setup_movement_button(
    state: Rc<RefCell<GameState>>,
    button: HtmlButtonElement,
    direction: Direction,
) {
    let target = EventTarget::from(button.clone());
    {
        let state = state.clone();
        let direction = direction.clone();
        let callback = move |_: Event| start_move(&mut state.borrow_mut(), &direction);

        add_event_listener_with_callback(target.clone(), "mousedown", callback.clone());
        add_event_listener_with_callback(target.clone(), "touchstart", callback);
    }
    let state = state.clone();
    let callback = move |_: Event| stop_move(&mut state.borrow_mut(), &direction);

    add_event_listener_with_callback(target.clone(), "mouseup", callback.clone());
    add_event_listener_with_callback(target.clone(), "touchend", callback);
}
