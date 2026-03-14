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
        <div style:display="flex" style:height="80vh" style:width="100vw">
            <LeftNavigationBar/>
            <Outlet/>
        </div>
    }
}

#[component]
pub fn Art() -> impl IntoView {
    view! {
        <h1>
            "Art!"
        </h1>
    }
}
