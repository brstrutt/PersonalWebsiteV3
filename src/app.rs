use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
            class=("green", move || count.get() % 2 == 0)
            class=("red", move || count.get() % 2 == 1)
            style=("font-size", move || format!("{}px", count.get()))
        >
            "Click me: "
            {count}
        </button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}
