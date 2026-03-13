use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view!{
        <nav>
            <A href="/">"Home"</A>
            <br/>
            <A href="/other-page">"Other Page"</A>
        </nav>
    }
}
