use leptos::prelude::*;
use leptos_router::components::Outlet;
use crate::app::components::LeftNavigationBar;

pub mod maps;
pub use maps::Maps;
pub mod pixel_art;
pub use pixel_art::PixelArt;
pub mod warhammer;
pub use warhammer::Warhammer;


#[component]
pub fn Wrapper() -> impl IntoView {
    view! {
        <LeftNavigationBar />
        <main>
            <Outlet />
        </main>
    }
}

#[component]
pub fn Art() -> impl IntoView {
    view! { <h1>"Art!"</h1> }
}
