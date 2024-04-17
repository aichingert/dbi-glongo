use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{HomePage, WriteBlog, Post};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Stylesheet id="leptos" href="/pkg/ssr_modes.css"/>
        <Title text="Blogs"/>

        <Router fallback>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="post/:article" view=Post/>
                    <Route path="write-blog" view=WriteBlog/>
                </Routes>
            </main>
        </Router>
    }
}
