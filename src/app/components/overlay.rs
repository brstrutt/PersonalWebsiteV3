use leptos::prelude::*;

#[component]
pub fn overlay(children: Children) -> impl IntoView {
    view! { <div class="overlay">{children()}</div> }
}
