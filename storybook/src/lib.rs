mod stories;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use stories::{button::BUTTON_STORY, card::CARD_STORY, typography::TYPOGRAPHY_STORY};
use ui::component::*;

const STORIES: &[Story] = &[TYPOGRAPHY_STORY, BUTTON_STORY, CARD_STORY];

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

pub struct Story {
    name: &'static str,
    description: &'static str,
    component: &'static (dyn Fn() -> View),
}

#[derive(Params, PartialEq)]
struct StoryParams {
    story: Option<String>,
}

#[component]
fn RenderStory() -> impl IntoView {
    let params = use_params::<StoryParams>();

    let story_name = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|p| p.story.clone())
                .unwrap_or_default()
                .unwrap_or_default()
        })
    };

    let story = move || {
        STORIES
            .iter()
            .find(|Story { name, .. }| *name == story_name())
            .unwrap()
    };

    view! {
        <div class="grid gap-6 py-8">
            <H1>{story_name}</H1>
            <span class="text-sm text-muted-foreground">{move || { story().description }}</span>

            {move || { story().component }}
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Stylesheet id="leptos" href="/pkg/storybook.css"/>
            <Title text="Storybook for vleterbees"/>

            <div class="flex min-h-screen w-full flex-col bg-slate-50">
                <header class="sticky top-0 flex h-16 items-center gap-4 border-b bg-background px-4 md:px-6">
                    <span class="text-lg font-semibold">"vleterbees Storybook"</span>
                </header>
                <main class="mx-auto grid w-full max-w-6xl items-start gap-6 md:grid-cols-[180px_1fr] lg:grid-cols-[180px_1fr]">
                    <div class="min-h-full py-8 border-r">
                        <nav class="grid items-start gap-4 text-sm text-muted-foreground">
                            <For
                                each=move || STORIES
                                key=|Story { name, .. }| name
                                children=move |Story { name, .. }: &Story| {
                                    view! {
                                        <A
                                            href=move || { (*name).to_owned() }
                                            class="aria-current:font-semibold aria-current:text-primary"
                                        >

                                            {*name}
                                        </A>
                                    }
                                }
                            />

                        </nav>
                    </div>
                    <Routes>
                        <Route path="/" view=|| view! { <h1>"Select a story"</h1> }/>
                        <Route path="/:story" view=RenderStory/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
