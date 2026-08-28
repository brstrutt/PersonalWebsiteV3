use leptos::leptos_dom;
use std::{cell::RefCell, rc::Rc};
use wasmfenbein3d::core::{motion, state::GameState};

pub fn character_motion_loop(state: Rc<RefCell<GameState>>) {
    {
        let mut state = state.borrow_mut();

        // Use time delta to control for framerate variations
        let current_time = leptos_dom::helpers::window().performance().unwrap().now();
        let time_since_last_frame_ms = current_time - state.last_frame_time_ms;

        let time_since_last_frame_s = time_since_last_frame_ms / 1000.0;

        // Calculate movement speed
        let velocity_per_s = if state.input.sprint { 12.0 } else { 4.0 };
        let velocity = velocity_per_s * time_since_last_frame_s;

        // Calculate the direction the player is facing
        let camera_rotation = state.world.camera.ray.get_angle();
        let motion = state
            .input
            .get_cameraspace_movement_direction()
            .rotate(camera_rotation)
            * velocity;

        // Move the character
        state.world.camera.ray.origin =
            motion::move_object(state.world.camera.ray.origin, &motion, &state.world);

        // Rotate the camera
        const ROTATION_SPEED: f64 = 0.001;

        let camera_rotation = state.input.camera_rotation;
        state.input.camera_rotation = 0;

        if camera_rotation != 0 {
            state.world.camera = state
                .world
                .camera
                .rotate(camera_rotation as f64 * ROTATION_SPEED);
        }
        state.world.camera.refresh_screen_rays();

        // Track the last time we ran a physics tick
        state.last_frame_time_ms = current_time;

        // Track how much time is passing between each tick (for FPS calculation/display)
        state.last_time_between_frames_ms = time_since_last_frame_ms;
    }

    leptos_dom::helpers::request_animation_frame(|| character_motion_loop(state));
}
