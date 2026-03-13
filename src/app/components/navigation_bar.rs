use leptos::prelude::*;
use crate::app::components::InternalLink;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view! {
        <nav>
            <InternalLink href="/art">"Art"</InternalLink>
            <InternalLink href="/">"Ben!"</InternalLink>
            <InternalLink href="/projects">"Projects"</InternalLink>
        </nav>
    }
}
