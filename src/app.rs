use crate::methods::{EntryDto, AuthorDto, CommentDto, get_all_entries};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

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
                    <Route path="write-blog" view=WriteBlog/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn WriteBlog() -> impl IntoView {
    view! {}
}

#[component]
fn HomePage() -> impl IntoView {
    let entry = create_resource(|| (), |_| async { get_all_entries().await });

    let entry_view = move || {
        entry.and_then(|entries| {
            entries
                .iter()
                .map(|entry| {
                    let category_view = entry.categories.iter()
                        .filter(|&s| matches!(
                            s.as_str(), "NASA" | "vulnerability" | "Programming" | "news" | "bypass"
                        ))
                        .map(|category| view! {
                            <a class="{category}" href="/category/{category}"> {category}</a>
                        })
                        .collect_view();

                    let last_edited = entry.creation_date
                        .to_chrono()
                        .to_rfc2822()
                        .chars()
                        .take(16)
                        .collect::<String>();

                    view! {
                        <p style="font-style: italic; font-size: 15px; color: #3f3f46"> { last_edited } </p>
                        <div>
                            <p> { &entry.title } </p>
                            { category_view }
                        </div>
                        <p style="color: #3f3f46"> { &entry.description } </p>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <div style="display: grid;">
                <div style="width: 100%; grid-column-start: 1; grid-column-end: 2; align-items: center">
                    <div style="display: flex; flex-direction: column; align-items: center">
                        <div>
                            <h1>Blogs</h1>
                        </div>

                        <div style="margin: 10px">
                            <a style="margin: 10px" href="/">Read</a>
                            <a style="margin: 10px" href="/">Write</a>
                        </div>
                    </div>
                </div>
                <div style="grid-column-start: 2; grid-column-end: 3">
                    <p>Recent posts</p>
                    <div style="border-bottom: 1px solid #3f3f46" />
                    <ul>{entry_view}</ul>
                </div>
            </div>

        </Suspense>
    }
}

#[component]
fn CommentView(comments_allowed: bool, comments: Vec<CommentDto>) -> impl IntoView {
    view! {
        {move || match comments_allowed {
            true => view! {
                <form action="" method="post" style="margin: 25px">
                    <div>
                        <textarea style="font-size:1.2em;"></textarea>
                    </div>
                <input type="submit" value="Submit" />
                </form>
            }.into_any(),
            false => view! { <div /> }.into_any()
        }}

        <div style="margin: 5px" />

        <For each=move || comments.clone()
            key=|comment| comment.creation_date.clone()
            children=move |comment: CommentDto| {
              view! {
                  <p>{ comment.text }</p>
              }
            }
        />
    }
}
