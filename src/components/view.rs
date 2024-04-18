use leptos::*;
use leptos_router::*;

use crate::database::dtos::CommentDto;
use crate::server::{get_entry, PostError};

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    article: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
    let query = use_params::<PostParams>();
    let article = move || query.with(|q| q.as_ref().map(|q| q.clone().article.unwrap_or_default()).map_err(|_| PostError::InvalidArticle));

    let post_resource = create_resource(article, |article| async move {
        match article {
            Ok(article) => get_entry(article).await.map(|data| data.ok_or(PostError::PostNotFound)).map_err(|_| PostError::ServerError),
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

            let category_view = entry.categories.iter()
                .map(|category| view! { <pre> {category} </pre> } )
                .collect_view();

            let coordinate_view = entry.content.coordinates
                .iter()
                .map(|link| view! { <p>Coordinate: long = {link.long} lat {link.lat} </p> })
                .collect_view();

            let article = entry.title.split(' ').collect::<Vec<_>>().join("-");
            let date = entry.creation_date
                .to_chrono()
                .to_rfc2822()
                .chars()
                .take(16)
                .collect::<String>();

            view! {
                <ul> { image_view } </ul>

                <p class="small-text">{ &date }</p>

                <h1>{&entry.title}</h1>
                <h2 style="font-style: italic;">{&entry.description} - { entry.impression_count } </h2>

                <ul> { category_view } </ul>

                <pre style="text-wrap: wrap"> {&entry.content.text} </pre>

                <div style="display: flex">
                    <pre>written by </pre>
                    <p style="font-size: 25px; color: #74c1c7">{ &entry.author.username }</p>
                </div>

                <ul> { link_view } </ul>
                <ul> { coordinate_view } </ul>

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
pub fn CommentView(_article: String, comments_allowed: bool, comments: Vec<CommentDto>) -> impl IntoView {

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
