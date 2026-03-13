use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view!{
        <nav>
            <A href="/art">"Art"</A>
            <A href="/">"Ben!"</A>
            <A href="/projects">"Projects"</A>
        </nav>
    }
}
