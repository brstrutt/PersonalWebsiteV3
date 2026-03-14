use leptos::prelude::*;
use leptos_router::components::Outlet;
use crate::app::components::RightNavigationBar;

pub mod guides;
pub use guides::Guides;
pub mod websites;
pub use websites::Websites;

#[component]
pub fn Wrapper() -> impl IntoView {
    view! {
        <main>
            <Outlet />
        </main>
        <RightNavigationBar />
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! { <h1>"Projects!"</h1> }
}
