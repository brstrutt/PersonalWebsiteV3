use leptos::prelude::*;
use leptos_router::components::A;
use crate::app::utils::internal_path;

#[component]
pub fn TopNavigationBar() -> impl IntoView {
    view! {
        <nav class="top-navigation-bar">
            <A href=internal_path("/art")>"Art"</A>
            <A href=internal_path("/") exact=true>
                "Ben!"
            </A>
            <A href=internal_path("/projects")>"Projects"</A>
        </nav>
    }
}

#[component]
pub fn LeftNavigationBar() -> impl IntoView {
    view! {
        <nav class="left-navigation-bar">
            <A href=internal_path("/art/warhammer")>"Warhammer"</A>
            <A href=internal_path("/art/maps")>"Maps"</A>
            <A href=internal_path("/art/pixel-art")>"Pixel Art"</A>
        </nav>
    }
}

#[component]
pub fn RightNavigationBar() -> impl IntoView {
    view! {
        <nav class="right-navigation-bar">
            <A href=internal_path("/projects/guides")>"Guides"</A>
            <A href=internal_path("/projects/websites")>"Websites"</A>
        </nav>
    }
}
