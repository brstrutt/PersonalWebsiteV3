use std::{cell::RefCell, rc::Rc};

use leptos::{html::Canvas, prelude::*};

mod world_display;
use leptos_router::hooks::use_navigate;
use wasmfenbein3d::{render::screen_buffer::ScreenBuffer, state::{State, textures::TextureLibrary, world}};
use world_display::textures;
use world_display::WorldDisplay;
mod touchscreen_button;
use touchscreen_button::TouchscreenButton;
mod d_pad;
use d_pad::DPad;
mod left_joycon;
use left_joycon::LeftJoycon;
mod right_joycon;
use right_joycon::RightJoycon;
mod a_button;
use a_button::AButton;

use crate::app::{components::game_ui::world_display::controls, utils::LoggerWithDuration};

#[derive(Copy, Clone, PartialEq)]
enum WorldDisplayState {
    SplashScreen,
    Loading,
    Wasmfenbein3dLoaded,
}

#[component]
pub fn game_ui() -> impl IntoView {
    let world_display_ref = NodeRef::<Canvas>::new();
    let (world_display_state, set_world_display_state) = signal(WorldDisplayState::SplashScreen);

    let texture_count = textures::TILING_TEXTURES.len() + textures::TEXTURES.len();
    let (loaded_textures, set_loaded_textures) = signal::<usize>(0);
    let textures = LocalResource::<TextureLibrary>::new(move || {
        textures::load_async_tracking_progress(set_loaded_textures)
    });

    let navigate = use_navigate();
    let (game_state, set_game_state) = signal_local::<Option<Rc<RefCell<State>>>>(None);
    Effect::new(move |_| {
        if let Some(element) = world_display_ref.get() {
            textures.with(|loaded_textures| {
                if let Some(textures) = loaded_textures && world_display_state.get() == WorldDisplayState::Loading{
                    let mut logger = LoggerWithDuration::new();
                    logger.log_time("Initialising wasmfenbein3d!");

                    // Setup the screen buffer we render to
                    let screen_width = element.width() as usize;
                    let screen_height = element.height() as usize;
                    let screen_buffer = ScreenBuffer::setup(screen_width, screen_height);
                    logger.log_time("Screen buffer created...");

                    // Setup the wasmfenbein3d state object
                    let state = Rc::new(RefCell::new(State::setup(
                        screen_buffer,
                        world::World::load(
                            &textures,
                            include_str!("game_ui/world_display/world/data.json"),
                        ),
                    )));
                    state.borrow_mut().input.pointer_locked = window()
                        .document()
                        .expect("no global `document` exists")
                        .pointer_lock_element()
                        .is_some();
                    logger.log_time("Game state initialised...");

                    // Setup the render loop
                    wasmfenbein3d::wasmrender::setup(state.clone(), element.clone());
                    logger.log_time("Render loop setup...");

                    // Setup controls
                    wasmfenbein3d::wasmcontrols::setup(
                        state.clone(),
                        element.clone(),
                        |id: &str| {
                            controls::on_click(id, navigate.clone());
                        },
                        || {
                            controls::on_mouse_capture(navigate.clone());
                        },
                    );
                    logger.log_time("Wasm controls setup...");
                    logger.log_time("Wasmfenbein3d initialised!");

                    set_world_display_state.set(WorldDisplayState::Wasmfenbein3dLoaded);
                    set_game_state.set(Some(state));
                }
            });
        }
    });

    view! {
        <div class="game_ui">
            <div class="game_screen">
                <WorldDisplay
                    node_ref=world_display_ref
                    state=world_display_state
                    set_state=set_world_display_state
                    textures_to_load=texture_count
                    textures_loaded=loaded_textures
                />
            </div>
            <div class="joycons">
                <LeftJoycon />
                <div class="spacer" />
                <RightJoycon game_state=game_state />
            </div>
        </div>
    }
}
