use std::{cell::RefCell, rc::Rc};

use leptos::{html::Div, prelude::*};
use wasmfenbein3d::state::State;
use web_sys::{EventTarget, TouchEvent};

use crate::app::utils::add_event_listener_with_callback;

#[component]
pub fn camera_slider(
    game_state: ReadSignal<Option<Rc<RefCell<State>>>, LocalStorage>,
) -> impl IntoView {
    let node_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        if let Some(element) = node_ref.get() && let Some(game_state) = game_state.get() {
            let cloned_state = game_state.clone();
            add_event_listener_with_callback(
                EventTarget::from(element.clone()),
                "touchstart",
                move |e: TouchEvent| {
                    e.prevent_default();
                    let mut state = cloned_state.borrow_mut();
                    let touch_points = e.target_touches();
                    if touch_points.length() > 0 {
                        let touch_x_position = touch_points
                            .item(0)
                            .expect("Failed to get first touch point on the camera_slider")
                            .screen_x();
                        state.input.last_canvas_touch_point_x = Some(touch_x_position);
                    }
                }
            );

            let cloned_state = game_state.clone();
            add_event_listener_with_callback(
                EventTarget::from(element.clone()),
                "touchmove",
                move |e: TouchEvent| {
                    e.prevent_default();
                    let mut state = cloned_state.borrow_mut();
                    const ACCELERATION: i32 = 10;

                    let touch_points = e.target_touches();
                    if touch_points.length() > 0 {
                        let touch_x_position = touch_points
                            .item(0)
                            .expect("Failed to get first touch point on the camera_slider")
                            .screen_x();

                        if state.input.last_canvas_touch_point_x.is_some() {
                            state.input.camera_rotation =
                                (state.input.last_canvas_touch_point_x.unwrap()
                                    - touch_x_position)
                                    * ACCELERATION;
                        }

                        state.input.last_canvas_touch_point_x = Some(touch_x_position);
                    }
                },
            );

            let cloned_state = game_state.clone();
            add_event_listener_with_callback(
                EventTarget::from(element),
                "touchend",
                move |e: TouchEvent| {
                    e.prevent_default();
                    let mut state = cloned_state.borrow_mut();
                    state.input.last_canvas_touch_point_x = None;
                },
            );
        }
    });

    view! { <div node_ref=node_ref class="camera_slider" /> }
}
