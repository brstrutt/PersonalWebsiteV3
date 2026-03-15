use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes};

use crate::app::components::MarkdownContent;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/embed-presentation-in-hugo") view=Page /> }
        .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <MarkdownContent markdown_text=include_str!("embed_presentation_in_hugo.md") />
    }
}
