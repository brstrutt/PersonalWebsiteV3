use crate::app::components::ProgressBar;
use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, components::Route, path};


#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/other-page") view=Page /> }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main>
            <p>"Other Page!"</p>
            <button
                on:click=move |_| *set_count.write() += 1
                class=("bold", move || count.get() % 2 == 0)
                style=("font-size", move || format!("{}px", count.get() + 10))
            >
                "Click me: "
                {count}
            </button>
            <ProgressBar progress=move || count.get() />
            <ProgressBar max=10 progress=count />
        </main>
    }
}
