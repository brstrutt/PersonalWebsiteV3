use crate::app::{
    components::world_display::textures::{
        PAINTINGS_PIXEL_ART_VERMINTIDE_TAPESTRY, PAINTING_GUIDE_CONTROL_DARK_SOULS_WITH_A_PIANO,
        PAINTING_GUIDE_EMBED_PRESENTATION_IN_HUGO,
        PAINTING_GUIDE_FIRST_TURN_KILL_BOSSES_IN_BRAVELY_DEFAULT2,
        PAINTING_GUIDE_PAYDAY2_SAFEHOUSE_NIGHTMARE,
        PAINTING_GUIDE_RIDGE_RACER_DS_UNLOCK_BURNING_NIGHTMARE,
    },
    pages,
};
use leptos::prelude::*;
use leptos_router::NavigateOptions;

pub fn on_mouse_capture(navigate: impl Fn(&str, NavigateOptions) + Clone) {
    navigate(pages::explore::PAGE_PATH, Default::default());
}

pub fn on_click(item_id: &str, navigate: impl Fn(&str, NavigateOptions) + Clone) {
    if let Some(dest_page) = navigate_to(item_id) {
        document().exit_pointer_lock();
        navigate(dest_page, Default::default());
    }
}

fn navigate_to(item_id: &str) -> Option<&'static str> {
    match item_id {
        val if val == PAINTINGS_PIXEL_ART_VERMINTIDE_TAPESTRY.id => {
            Some(pages::art::pixel_art::vermintide::PAGE_PATH)
        }
        val if val == PAINTING_GUIDE_RIDGE_RACER_DS_UNLOCK_BURNING_NIGHTMARE.id => {
            Some(pages::projects::guides::ridge_racer_ds_unlock_burning_nightmare::PAGE_PATH)
        }
        val if val == PAINTING_GUIDE_FIRST_TURN_KILL_BOSSES_IN_BRAVELY_DEFAULT2.id => {
            Some(pages::projects::guides::first_turn_kill_bosses_in_bravely_default2::PAGE_PATH)
        }
        val if val == PAINTING_GUIDE_PAYDAY2_SAFEHOUSE_NIGHTMARE.id => {
            Some(pages::projects::guides::payday2_safehouse_nightmare::PAGE_PATH)
        }
        val if val == PAINTING_GUIDE_CONTROL_DARK_SOULS_WITH_A_PIANO.id => {
            Some(pages::projects::guides::control_dark_souls_with_a_piano::PAGE_PATH)
        }
        val if val == PAINTING_GUIDE_EMBED_PRESENTATION_IN_HUGO.id => {
            Some(pages::projects::guides::embed_presentation_in_hugo::PAGE_PATH)
        }
        &_ => None,
    }
}
