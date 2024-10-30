use leptos::*;
use leptos_router::*;



#[component]
pub fn parent_child_home() -> impl IntoView {
    view! {
        <div>
            <h2>"Parent Child Home"</h2>
            <ul>
                <li>
                <a href="/parent-child/write-signal">"Write Signal"</a>
                </li>
            </ul>
            <Outlet/>
        </div>
    }
}