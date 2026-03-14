use leptos::prelude::*;
use leptos_router::components::A;
use crate::app::URL_SUFFIX;

#[component]
pub fn InternalLink
(
    href: &'static str,
    children: Children,
) -> impl IntoView {
    view!{
        <A
            href=format!("{}{}", URL_SUFFIX, href)
        >
            {children()}
        </A>
    }
}
