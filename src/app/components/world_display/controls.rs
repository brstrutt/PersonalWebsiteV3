use super::world::entity_ids::*;
use crate::app::pages;
use crate::app::utils::add_event_listener_with_callback;
use leptos::ev::{self, Event, KeyboardEvent, MouseEvent, TouchEvent};
use leptos::leptos_dom;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{Element, EventTarget, HtmlCanvasElement};
use leptos_router::NavigateOptions;
use std::{cell::RefCell, rc::Rc};
use wasmfenbein3d::core::state::GameState;

mod movement;
use movement::*;

pub fn setup_controls(
    canvas_element: HtmlCanvasElement,
    state: Rc<RefCell<GameState>>,
    navigate: impl Fn(&str, NavigateOptions) + Clone,
) {
    setup_camera_mouse_controls(canvas_element.clone(), state.clone(), navigate.clone());
    setup_camera_touch_control(canvas_element.clone(), state.clone(), navigate.clone());
    setup_keyboard_movement_controls(state.clone());
    setup_touchscreen_movement_controls(state.clone());
    setup_click_passthrough(canvas_element, state, navigate);
}

fn setup_camera_mouse_controls(
    canvas_element: HtmlCanvasElement,
    state: Rc<RefCell<GameState>>,
    navigate: impl Fn(&str, NavigateOptions) + Clone,
) {
    // Lock the mouse to the canvas on click
    let target = EventTarget::from(canvas_element.clone());
    add_event_listener_with_callback(target, "click", move |_: Event| {
        canvas_element.request_pointer_lock();
    });

    // Track if the mouse is locked to the canvas or not, and stop all movement when the mouse becomes unlocked
    let cloned_state = state.clone();
    add_event_listener_with_callback(
        EventTarget::from(document()),
        "pointerlockchange",
        move |_: Event| {
            let mut state = cloned_state.borrow_mut();
            state.input.pointer_locked = document().pointer_lock_element().is_some();
            if state.input.pointer_locked {
                navigate(pages::explore::PAGE_PATH, Default::default());
            } else {
                reset_movement(&mut state);
            }
        },
    );

    // Convert mouse movement into camera movement if the mouse is locked to the canvas element
    let cloned_state = state.clone();
    leptos_dom::helpers::window_event_listener(ev::mousemove, move |e: MouseEvent| {
        let mut state = cloned_state.borrow_mut();

        if state.input.pointer_locked {
            state.input.camera_rotation += e.movement_x();
        }
    });
}

fn setup_camera_touch_control(
    canvas_element: HtmlCanvasElement,
    state: Rc<RefCell<GameState>>,
    navigate: impl Fn(&str, NavigateOptions) + Clone,
) {
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

                navigate(pages::explore::PAGE_PATH, Default::default());
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

fn setup_keyboard_movement_controls(state: Rc<RefCell<GameState>>) {
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

pub mod input_action_keys {
    pub const DATA_ACTION_MOVE_LEFT: &str = "character_input_move_left";
    pub const DATA_ACTION_MOVE_RIGHT: &str = "character_input_move_right";
    pub const DATA_ACTION_MOVE_FORWARD: &str = "character_input_move_forward";
    pub const DATA_ACTION_MOVE_BACKWARD: &str = "character_input_move_backward";
}
use input_action_keys::*;

fn setup_touchscreen_movement_controls(state: Rc<RefCell<GameState>>) {
    setup_movement_button(state.clone(), DATA_ACTION_MOVE_FORWARD, Direction::Forward);
    setup_movement_button(
        state.clone(),
        DATA_ACTION_MOVE_BACKWARD,
        Direction::Backward,
    );
    setup_movement_button(state.clone(), DATA_ACTION_MOVE_LEFT, Direction::Left);
    setup_movement_button(state.clone(), DATA_ACTION_MOVE_RIGHT, Direction::Right);
}

fn setup_movement_button(
    state: Rc<RefCell<GameState>>,
    input_element_data_action: &'static str,
    direction: Direction,
) {
    {
        let state = state.clone();
        let direction = direction.clone();

        window_event_listener(ev::mousedown, move |event: MouseEvent| {
            if does_event_target_have_data_action(event.target(), input_element_data_action) {
                start_move(&mut state.borrow_mut(), &direction);
            }
        });
    }
    {
        let state = state.clone();
        let direction = direction.clone();
        window_event_listener(ev::touchstart, move |event: TouchEvent| {
            if does_event_target_have_data_action(event.target(), input_element_data_action) {
                start_move(&mut state.borrow_mut(), &direction);
            }
        });
    }

    {
        let state = state.clone();
        let direction = direction.clone();

        window_event_listener(ev::mouseup, move |event: MouseEvent| {
            if does_event_target_have_data_action(event.target(), input_element_data_action) {
                stop_move(&mut state.borrow_mut(), &direction);
            }
        });
    }
    {
        let state = state.clone();
        let direction = direction.clone();

        window_event_listener(ev::touchend, move |event: TouchEvent| {
            if does_event_target_have_data_action(event.target(), input_element_data_action) {
                stop_move(&mut state.borrow_mut(), &direction);
            }
        });
    }
}

fn does_event_target_have_data_action(
    event_target: Option<EventTarget>,
    data_action: &'static str,
) -> bool {
    if let Some(target) = event_target {
        if let Ok(element) = target.dyn_into::<Element>() {
            if let Some(element_data_action) = element.get_attribute("data-action") {
                return element_data_action == data_action;
            }
        }
    }
    false
}

fn setup_click_passthrough(
    canvas_element: HtmlCanvasElement,
    state: Rc<RefCell<GameState>>,
    navigate: impl Fn(&str, NavigateOptions) + Clone,
) {
    let canvas_target = EventTarget::from(canvas_element);
    let cloned_state = state.clone();
    let cloned_navigate = navigate.clone();
    add_event_listener_with_callback(canvas_target.clone(), "click", move |_e: MouseEvent| {
        let state = cloned_state.borrow();

        if state.input.pointer_locked {
            let item_ids = state.input.get_items_under_cursor(&state.world);
            for id in item_ids {
                on_click(id.as_str(), cloned_navigate.clone());
            }
        }
    });

    let cloned_state = state.clone();
    add_event_listener_with_callback(canvas_target.clone(), "touchmove", move |_e: TouchEvent| {
        let mut state = cloned_state.borrow_mut();

        state.input.touch_has_moved_camera = true;
    });

    add_event_listener_with_callback(canvas_target.clone(), "touchend", move |_e: TouchEvent| {
        let mut state = state.borrow_mut();

        if !state.input.touch_has_moved_camera {
            if state.input.pointer_locked {
                let item_ids = state.input.get_items_under_cursor(&state.world);
                for id in item_ids {
                    on_click(id.as_str(), navigate.clone());
                }
                state.input.pointer_locked = false
            } else {
                state.input.pointer_locked = true;
            }
        }
        state.input.touch_has_moved_camera = false;
    });
}

fn on_click(item_id: &str, navigate: impl Fn(&str, NavigateOptions) + Clone) {
    if let Some(dest_page) = navigate_to(item_id) {
        document().exit_pointer_lock();
        navigate(dest_page, Default::default());
    }
}

fn navigate_to(item_id: &str) -> Option<&'static str> {
    match item_id {
        VERMINTIDE_TAPESTRY_ID => Some(pages::art::pixel_art::vermintide::PAGE_PATH),
        RIDGE_RACER_BURNING_NIGHTMARE_ID => {
            Some(pages::projects::guides::ridge_racer_ds_unlock_burning_nightmare::PAGE_PATH)
        }
        FIRST_TURN_KILL_BOSSES_IN_BRAVELY_DEFAULT2_ID => {
            Some(pages::projects::guides::first_turn_kill_bosses_in_bravely_default2::PAGE_PATH)
        }
        PAYDAY2_SAFEHOUSE_NIGHTMARE_ID => {
            Some(pages::projects::guides::payday2_safehouse_nightmare::PAGE_PATH)
        }
        CONTROL_DARK_SOULS_WITH_A_PIANO_ID => {
            Some(pages::projects::guides::control_dark_souls_with_a_piano::PAGE_PATH)
        }
        EMBED_PRESENTATION_IN_HUGO_ID => {
            Some(pages::projects::guides::embed_presentation_in_hugo::PAGE_PATH)
        }
        &_ => None,
    }
}
