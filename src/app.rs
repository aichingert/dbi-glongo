use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Stylesheet id="leptos" href="/pkg/ssr_modes.css"/>
        <Title text="Blogs"/>

        <Router fallback>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let entries = create_resource(|| (), |_| async { get_all_entries().await });

    let entries_view = move || {
        entries.and_then(|entry| {
            entry.iter()
                .map(|entry| view! { 
                    <p>{&entry.title}</p>
                    <p>{&entry.description}</p>
                })
                .collect_view()
        })
    };

    view! {
        <h1>"Blogs"</h1>
        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <ul>{entries_view}</ul>
        </Suspense>
    }
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidId,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    title: String,
    author: bson::oid::ObjectId,
    description: String,

    creation_date: bson::DateTime,
    edit_dates: Vec<bson::DateTime>,
    impression_count: u64,

    comments_allowed: bool,
    categories: Vec<String>,
    comments: Vec<String>,
}

#[server]
pub async fn get_all_entries() -> Result<Vec<Entry>, ServerFnError> {
    use futures_util::StreamExt;

    let client = mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin").await?;
    let cursor = client.database("blogDB").collection::<Entry>("entries").find(None, None).await?;

    Ok(cursor.collect::<Vec<Result<Entry, _>>>().await.into_iter().flatten().collect::<Vec<Entry>>())
}
