use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/vleterbees.css"/>
        <Title text="vleterbees"/>

        <div>
            <h1 class="text-3xl font-bold underline">"Welcome to vleterbees!"</h1>
        </div>
    }
}
