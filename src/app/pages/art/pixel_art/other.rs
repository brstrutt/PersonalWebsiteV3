use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, components::Route, path};

use crate::app::utils::internal_path;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/other") view=Page /> }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <h1>"Other"</h1>
        <p>"Here's some pixel art that doesn't really fit into the other categories."</p>
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/postman.png")
            title="The Postman!"
            alt="Pixel art of the postman from Zelda: Twilight Princess"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/pokemon.png")
            title="Pokemon"
            alt="Pixel art of a few small pokemon"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/sprout_run.gif")
            title="Bellsprout"
            alt="Pixel art gif of a bellsprout running"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/spoink_boink.gif")
            title="Spoink"
            alt="Pixel art gif of a spoink bouncing"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/bradley.gif")
            title="Bradley"
            alt="Pixel art of Fuhrer: King Bradley"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/2024_01_21_fall.png")
            title="The fall"
            alt="Pixel art of Sheeta falling from the sky"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/zant.png")
            title="Zant"
            alt="Pixel art of Zant from Zelda: Twilight Princess"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/dark_doorway.png")
            title="Doorway and Face"
            alt="Pixel art of a lit up doorway and a spooky face"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/2024_02_02_mystical.png")
            title="Mystical"
            alt="Pixel art of some circles that look kinda magical"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/2024_01_18_skill.png")
            title="Skill Icons"
            alt="Pixel art of some icons that could represent skills in a videogame"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/2024_01_15_magician.png")
            title="Magicians"
            alt="Pixel art of some magicians"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/2024_01_14_slime.png")
            title="Slimes"
            alt="Pixel art of some slimes"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/2024_01_24_researchl.png")
            title="Research"
            alt="Pixel art of a man doing...science?"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/other/2024_01_26_metal.png")
            title="Metal"
            alt="Pixel art practice of creating things that look like metal"
            style:border="1px solid black"
        />
    }
}