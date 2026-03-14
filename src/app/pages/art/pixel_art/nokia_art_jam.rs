use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, components::Route, path};

use crate::app::utils::internal_path;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/nokia-art-jam-3") view=Page /> }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <h1>"Nokia Art Jam 3"</h1>
        <p>"In April 2025 I took part in the Nokia Art Jam."</p>
        <p>
            "The theme was \"What is James Bond playing on his Nokia 3410?\". All submissions had to be 96x65px in size, using only pure black and pure white pixels."
        </p>
        <p>"Here are the three submissions I ended up coming up with."</p>
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/nokia_art_jam/1_keyboard_cat.png")
            title="Keyboard Cat"
            alt="1 bit pixel art of piano cat playing on youtube"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/nokia_art_jam/2_worms.png")
            title="Worms Open Warfare"
            alt="1 bit pixel art of Worms Open Warfare"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/nokia_art_jam/3_house.png")
            title="House MD"
            alt="1 bit pixel art of a House MD game in which you diagnose Lupus to all patients"
            style:border="1px solid black"
        />
        <br />
        <a href="https://itch.io/jam/nokiaartjam-3" target="_blank" rel="noopener noreferrer">
            "Click here to go to the Nokia Art Jam 3 page on itch.io"
        </a>
    }
}
