use leptos::*;


#[component]
pub fn parent_child_home() -> impl IntoView {
    view! {
        <div>
            <h2>"Parent Child Home:"</h2>
            <p>"This is the parent child home page."</p>
        </div>
    }
}