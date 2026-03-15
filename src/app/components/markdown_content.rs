use leptos::prelude::*;
use markdown::Options;

#[component]
pub fn MarkdownContent(
    markdown_text: &'static str
) -> impl IntoView {
    let mut temp = Options::gfm();
    temp.parse.constructs.frontmatter = true;
    let html = markdown::to_html_with_options(
        markdown_text,
        &temp,
    )
    .unwrap_or("Failed to parse markdown document.".to_owned());

    view! { <div inner_html=html /> }
}