use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes};

use crate::app::components::MarkdownContent;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/control-dark-souls-with-piano") view=Page /> }
        .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <MarkdownContent markdown_text=include_str!("control_dark_souls_with_a_piano.md") />
    }
}
