use leptos::*;
use leptos_router::ActionForm;

use crate::server::AddPost;
use crate::components::utils::{FileUpload, SelectOption};

#[component]
pub fn WriteBlog() -> impl IntoView {
    let (sf, ssf) = create_signal(None::<String>);
    let (value, set_value) = create_signal("news".to_string());
    let add_post = Action::<AddPost, _>::server();

    view! {
        <div style="display: grid;">
            <div style="width: 100%; grid-column-start: 1; grid-column-end: 2; align-items: center">
                <div style="display: flex; flex-direction: column; align-items: center">
                    <div style="margin: 10px">
                        <h1>Blogs</h1>

                        <a style="margin: 10px" href="/">Read</a>
                        <a style="margin: 10px" href="/write-blog">Write</a>
                    </div>
                </div>
            </div>
            <div style="grid-column-start: 2; grid-column-end: 3; column-width: 1200px">
                <ActionForm action=add_post>
                    <div>
                        <select on:change=move |ev| {
                            let new_value = event_target_value(&ev);
                            set_value.set(new_value);
                        }>
                            <SelectOption value is="NASA"/>
                            <SelectOption value is="vulnerability"/>
                            <SelectOption value is="Programming"/>
                            <SelectOption value is="news"/>
                            <SelectOption value is="bypass"/>
                        </select>

                        <div class="input-item">
                            <label>
                                "Title: "
                            <input type="text" name="entry[title]" value="" />
                            </label>
                        </div>

                        <div class="input-item">
                            <label>
                                "Description: "
                            <input type="text" name="entry[description]" value="" />
                            </label>
                        </div>

                        <div class="input-item">
                            <label>
                                "Text: "
                            <input type="text" name="entry[text]" value="" />
                            </label>
                        </div>

                        <div class="input-item">
                            <label>
                                "Comments allowed: "
                            <input type="checkbox" name="entry[comments_allowed]" />
                            </label>
                        </div>

                        <div class="input-item">
                            <FileUpload ssf=ssf />
                            <input type="hidden" name="entry[image]" value=move || sf.get().unwrap_or_default() />
                        </div>

                        // author is hard coded, but should normaly be the one that is currently logged in
                        <input type="hidden" name="entry[author]" value="6620436a0c07f5db34c00e84" />
                        <input type="hidden" name="entry[category]" value=move || value.get() />
                        <div class="input-item">
                            <input type="submit" />
                        </div>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}
