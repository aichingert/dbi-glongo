use crate::methods::{get_all_entries, get_entry, CommentDto, PostError, AddPost};
use leptos::*;
use leptos::html::Input;
use leptos_meta::*;
use leptos_router::*;
use log::Level;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, js_sys};

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

#[component]
fn WriteBlog() -> impl IntoView {
    let (sf, ssf) = create_signal(None::<String>);
    let file_input = create_node_ref::<Input>();
    let on_file_change = move |_| {
        if let Some(files) = file_input.get().and_then(|f| f.dyn_ref::<HtmlInputElement>().unwrap().files()) {
            let file = files.get(0).unwrap();
            let file_blob_promise = js_sys::Promise::resolve(&file.array_buffer());

            spawn_local(async move {
                let bytes = wasm_bindgen_futures::JsFuture::from(file_blob_promise).await.unwrap();
                let byte_arr = js_sys::Uint8Array::new(&bytes);
                let s = serde_json::to_string(&byte_arr.to_vec()).unwrap();
                _ = console_log::init_with_level(Level::Debug);
                ssf.set(Some(s));
            });
        }
    };

    let add_post = Action::<AddPost, _>::server();

    view! {
        <input ref=file_input type="file" on:change=on_file_change/>
        <ActionForm action=add_post>
            <input type="hidden" name="entry[image]" value=move || sf.get().unwrap_or_default() />
            <input type="submit" />
        </ActionForm>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let entry = create_resource(|| (), |_| async { get_all_entries().await });

    let entry_view =
        move || {
            entry.and_then(|entries| {
                entries
                .iter()
                .map(|entry| {
                    let category_view = entry.categories.iter()
                        .filter(|&s| matches!(
                            s.as_str(), "NASA" | "vulnerability" | "Programming" | "news" | "bypass"
                        ))
                        .map(|category| view! {
                            <a class="{category}" href="" exact=true> {category}</a>
                        })
                        .collect_view();

                    let last_edited = entry.get_date_string();

                    view! {
                        <div class="blog">
                            <div class="small-text" style="font-size: 20px; display: flex; align-items: center;">
                                <p> { last_edited } </p>
                            </div>

                            <div style="display: flex; align-items: center;">
                                <p> { &entry.title } </p>
                                { category_view }
                            </div>
                            <p style="color: #514e4d"> { &entry.description } </p>
                            <p style="color: #74c1c7"> written by { &entry.author.username } </p>
                            <a href="/post/"{ &entry.title.split(' ').collect::<Vec<_>>().join("-") } > Read more </a>
                        </div>
                    }
                })
                .collect_view()
            })
        };

    view! {
        <Suspense fallback=move || view! { <p style="display: flex; justify-content: center">"Loading posts..."</p> }>
            <div style="display: grid;">
                <div style="width: 100%; grid-column-start: 1; grid-column-end: 2; align-items: center">
                    <div style="display: flex; flex-direction: column; align-items: center">
                        <div>
                            <h1>Blogs</h1>
                        </div>

                        <div style="margin: 10px">
                            <a style="margin: 10px" href="/">Read</a>
                            <a style="margin: 10px" href="/write-blog">Write</a>
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

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    article: Option<String>,
}

#[component]
fn Post() -> impl IntoView {
    let query = use_params::<PostParams>();
    let article = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.clone().article.unwrap_or_default())
                .map_err(|_| PostError::InvalidArticle)
        })
    };

    let post_resource = create_resource(article, |article| async move {
        match article {
            Ok(article) => get_entry(article)
                .await
                .map(|data| data.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError),
            Err(e) => Err(e),
        }
    });

    let post = move || match post_resource.get() {
        Some(Ok(Ok(v))) => Ok(v),
        _ => Err(PostError::ServerError),
    };

    let post_view = move || {
        post().map(|entry| {
            let image_view = entry.content.images
                .iter()
                .map(|image| view! {
                    <div style="display:flex; justify-content: center">
                        <img src={ format!("data:image/png;base64, {}", image) } />
                    </div>
                })
                .collect_view();
            let link_view = entry.content.links
                .iter()
                .map(|link| view! { <a href={link}>{link}</a><br/>})
                .collect_view();

            let article = entry.title.split(' ').collect::<Vec<_>>().join("-");

            view! {
                <ul> { image_view } </ul>

                <p class="small-text">{ &entry.get_date_string() }</p>

                <h1>{&entry.title}</h1>
                <h2 style="font-style: italic;">{&entry.description}</h2>

                <pre style="text-wrap: wrap"> {&entry.content.text} </pre>

                <div style="display: flex">
                    <pre>written by </pre>
                    <p style="font-size: 25px; color: #74c1c7">{ &entry.author.username }</p>
                </div>

                <ul> { link_view } </ul>

                <CommentView _article=article comments_allowed=entry.comments_allowed comments=entry.comments.clone() />

                <div style="border-bottom: solid white 1px; margin: 25px" />
            }
        })
    };

    view! {
        <Suspense fallback=move || view! { <p style="display: flex; justify-content: center">"Loading post..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, error)| view! { <li>{error.to_string()} </li> })
                            .collect_view()
                        }
                        </ul>
                    </div>
                }
            }>

                <div style="display: grid;">
                    <div style="grid-column-start: 1; grid-column-end: 2; align-items: center">
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
                        { post_view }
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn CommentView(_article: String, comments_allowed: bool, comments: Vec<CommentDto>) -> impl IntoView {

    view! {
        {move || match comments_allowed {
            true => view! {
                <form action="/api/comments/{_article}" method="post" style="margin: 25px">
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
                  <div style="display:flex">
                        <pre style="color: #74c1c7"> { &comment.author.username } </pre>
                        <pre style="text-wrap: wrap"> commented: { comment.text } </pre>
                  </div>
              }
            }
        />
    }
}
