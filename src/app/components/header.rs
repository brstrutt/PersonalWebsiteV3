use leptos::prelude::*;

use crate::app::components::TopNavigationBar;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <TopNavigationBar />
        </header>
    }
}