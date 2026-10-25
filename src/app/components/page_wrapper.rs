use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn page_wrapper(children: Children) -> impl IntoView {
    view! {
        <main>
            <div class="page_wrapper">{children()}</div>
        </main>
    }
}

#[component]
pub fn leptos_routing_page_wrapper() -> impl IntoView {
    view! {
        <PageWrapper>
            <Outlet />
        </PageWrapper>
    }
}
