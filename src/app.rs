use leptos::prelude::*;
use leptos_router::{components::{A, Route, Router, Routes}, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <br/>
                <A href="/other-page">"Other Page"</A>
            </nav>
            <main>
                <Routes fallback=|| "404 Not found!">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/other-page") view=OtherPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <p>
            "HOME!"
        </p>
    }
}

/// A custom progress bar with custom default styling/values
#[component]
fn ProgressBar(
    /// The current value in the progress bar
    #[prop(into)]
    progress: Signal<i32>,
    /// The value at which the progress bar reaches 100%
    #[prop(default = 50)]
    max: u16,
) -> impl IntoView {
    view! { <progress max=max value=progress style=("display", "block") /> }
}


#[component]
fn OtherPage() -> impl IntoView {
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
