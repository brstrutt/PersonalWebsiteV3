use leptos::prelude::*;
use crate::app::components::InternalLink;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view! {
        <nav class="top-navigation-bar">
            <InternalLink href="/art">"Art"</InternalLink>
            <InternalLink href="/">"Ben!"</InternalLink>
            <InternalLink href="/projects">"Projects"</InternalLink>
        </nav>
    }
}

#[component]
pub fn LeftNavigationBar() -> impl IntoView {
    view! {
        <nav class="left-navigation-bar">
            <InternalLink href="/art/warhammer">"Warhammer"</InternalLink>
            <InternalLink href="/art/maps">"Maps"</InternalLink>
            <InternalLink href="/art/pixel-art">"Pixel Art"</InternalLink>
        </nav>
    }
}

#[component]
pub fn RightNavigationBar() -> impl IntoView {
    view! {
        <nav class="right-navigation-bar">
            <InternalLink href="/projects/guides">"Guides"</InternalLink>
            <InternalLink href="/projects/websites">"Websites"</InternalLink>
        </nav>
    }
}
