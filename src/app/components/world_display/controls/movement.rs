use wasmfenbein3d::core::state::GameState;

pub fn reset_movement(state: &mut GameState) {
    state.input.sprint = false;
    state.input.move_left = false;
    state.input.move_right = false;
    state.input.move_forward = false;
    state.input.move_backward = false;
}

#[derive(PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub fn key_to_direction(key: &str) -> Option<Direction> {
    match key {
        "a" | "A" => Some(Direction::Left),
        "d" | "D" => Some(Direction::Right),
        "w" | "W" => Some(Direction::Forward),
        "s" | "S" => Some(Direction::Backward),
        &_ => None,
    }
}

pub fn start_move(state: &mut GameState, direction: &Direction) {
    change_move(state, direction, MovementEvent::Start)
}

pub fn stop_move(state: &mut GameState, direction: &Direction) {
    change_move(state, direction, MovementEvent::Stop)
}

#[derive(PartialEq)]
enum MovementEvent {
    Start,
    Stop,
}

fn change_move(state: &mut GameState, direction: &Direction, event_type: MovementEvent) {
    let start_move = event_type == MovementEvent::Start;
    match direction {
        Direction::Left => state.input.move_left = start_move,
        Direction::Right => state.input.move_right = start_move,
        Direction::Forward => state.input.move_forward = start_move,
        Direction::Backward => state.input.move_backward = start_move,
    }
}
