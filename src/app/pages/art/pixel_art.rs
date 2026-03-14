use leptos::prelude::*;

pub mod nokia_art_jam;
pub use nokia_art_jam::NokiaArtJam;
pub mod vermintide;
pub use vermintide::Vermintide;

use leptos_router::components::A;
use crate::app::utils::internal_path;

#[component]
pub fn PixelArt() -> impl IntoView {
    view! {
        <h1>"Pixel Art!"</h1>
        <A href=internal_path("/art/pixel-art/nokia-art-jam-3")>"Nokia art jam 3"</A>
        <br/>
        <A href=internal_path("/art/pixel-art/vermintide")>"Warhammer Vermintide 2"</A>
    }
}
