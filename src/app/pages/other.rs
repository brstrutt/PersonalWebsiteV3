use crate::app::components::ProgressBar;
use leptos::prelude::*;

#[component]
pub fn Other() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <p>
            "Other Page!"
        </p>
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
    }
}
