use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <a href="https://github.com/brstrutt" target="_blank" rel="noopener noreferrer">
                <img src="assets/images/GitHub-Mark-Light-64px.png" title="Github" alt="Github Logo" width="32px" height="32px"/>
            </a>
            <a href="https://www.linkedin.com/in/ben-strutt-09b9a280/" target="_blank" rel="noopener noreferrer">
                <img src="assets/images/LI-In-Bug-Modified.png" title="LinkedIn" alt="LinkedIn Logo(Slightly Modified)" width="32px" height="32px"/>
            </a>
        </footer>
    }
}
