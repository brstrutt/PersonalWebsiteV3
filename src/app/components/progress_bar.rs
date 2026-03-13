use leptos::prelude::*;

/// A custom progress bar with custom default styling/values
#[component]
pub fn ProgressBar(
    /// The current value in the progress bar
    #[prop(into)]
    progress: Signal<i32>,
    /// The value at which the progress bar reaches 100%
    #[prop(default = 50)]
    max: u16,
) -> impl IntoView {
    view! { <progress max=max value=progress style=("display", "block") /> }
}
