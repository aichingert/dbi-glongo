use leptos::*;

use crate::server::get_all_entries;

#[component]
pub fn HomePage() -> impl IntoView {
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
                            <a class={category} href="" exact=true> {category}</a>
                        })
                            .collect_view();

                        let last_edited = entry.creation_date
                            .to_chrono()
                            .to_rfc2822()
                            .chars()
                            .take(16)
                            .collect::<String>();

                        let article = format!("/post/{}", entry.title.split(' ').collect::<Vec<_>>().join("-"));

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
                            <a href={article}> Read more </a>
                        </div>
                    }
                    })
                    .collect_view()
            })
        };

    view! {
        <Suspense fallback=move || view! { <p style="display: flex; justify-content: center">"Loading posts..."</p> }>
            <div style="display: grid;">
                <div style="width: 100%; grid-column-start: 1; grid-column-end: 2; column-width: 400px; align-items: center">
                    <div style="display: flex; flex-direction: column; align-items: center">
                        <div style="margin: 10px">
                            <h1>Blogs</h1>

                            <a style="margin: 10px" href="/">Read</a>
                            <a style="margin: 10px" href="/write-blog">Write</a>
                        </div>
                    </div>
                </div>
                <div style="grid-column-start: 2; grid-column-end: 3">
                    <div>
                        <p style="margin-left: 50px">Recent posts</p>
                        <div style="margin-left: 50px; border-bottom: 1px solid #3f3f46" />
                        <ul>{entry_view}</ul>
                    </div>
                </div>
            </div>

        </Suspense>
    }
}
