use leptos::prelude::*;

use crate::app::components::NavigationBar;

#[component]
pub fn Header() -> impl IntoView {
    view!{
        <header>
            <NavigationBar />
        </header>
    }
}