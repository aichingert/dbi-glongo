use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
                    <Route path="write-blog" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let entry = create_resource(|| (), |_| async { get_all_entries().await });

    let entry_view = move || {
        entry.and_then(|entries| {
            entries.iter()
                .map(|entry| {
                    let image_view = entry.content.images
                        .iter()
                        .map(|image| view! {
                            <img src={ format!("data:image/png;base64, {}", image) } />
                        })
                        .collect_view();
                    let link_view = entry.content.links
                        .iter()
                        .map(|link| view! { <a href={link}>{link}</a><br/>})
                        .collect_view();

                    view! { 
                        <ul> { image_view } </ul>
                        <h1>{&entry.title}</h1>
                        <p style="font-style: italic">{&entry.description}</p>

                        <pre style="text-wrap: wrap"> {&entry.content.text} </pre>

                        <ul> { link_view } </ul>

                        <CommentView comments_allowed=entry.comments_allowed comments=entry.comments.clone() />

                        <div style="border-bottom: solid white 1px; margin: 25px" />

                    }
                })
                .collect_view()
            })
    };

    view! {
        <h1>"Blogs"</h1>
        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <ul>{entry_view}</ul>
        </Suspense>
    }
}

#[component]
fn CommentView(comments_allowed: bool, comments: Vec<Comment>) -> impl IntoView {
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
            children=move |comment: Comment| {
              view! {
                  <p>{ comment.text }</p>
              }
            }
        />
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

    content: Content,

    comments_allowed: bool,
    categories: Vec<String>,
    comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    text: String,
    links: Vec<String>,
    coordinates: Vec<Coordinate>,
    images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    long: f64,
    lat: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    text: String,
    creation_date: bson::DateTime,
    author: bson::oid::ObjectId,
}

#[server]
pub async fn get_all_entries() -> Result<Vec<Entry>, ServerFnError> {
    use futures_util::StreamExt;

    let client = mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin").await?;
    let cursor = client.database("blogDB").collection::<Entry>("entries").find(None, None).await?;

    Ok(cursor.collect::<Vec<Result<Entry, _>>>().await.into_iter().flatten().collect::<Vec<Entry>>())
}
