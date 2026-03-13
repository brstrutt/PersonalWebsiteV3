use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <a href="https://github.com/brstrutt" target="_blank" rel="noopener noreferrer">"Github"</a>
            <a href="https://www.linkedin.com/in/ben-strutt-09b9a280/" target="_blank" rel="noopener noreferrer">"LinkedIn"</a>
        </footer>
    }
}
