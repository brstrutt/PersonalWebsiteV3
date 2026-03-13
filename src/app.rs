use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
            class=("bold", move || count.get() % 2 == 0)
            style=("font-size", move || format!("{}px", count.get()))
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="50"
            value=count
            style=("display", "block")
        />
    }
}
