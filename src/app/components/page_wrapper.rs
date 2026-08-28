use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn page_wrapper() -> impl IntoView {
    view! {
        <main>
            <div class="page_wrapper">
                <Outlet />
            </div>
        </main>
    }
}
