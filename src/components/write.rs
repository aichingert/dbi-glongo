use leptos::*;
use leptos_router::ActionForm;

use crate::server::AddPost;
use crate::components::utils::FileUpload;

#[component]
pub fn WriteBlog() -> impl IntoView {
    let (sf, ssf) = create_signal(None::<String>);
    let add_post = Action::<AddPost, _>::server();

    view! {
        <ActionForm action=add_post>
            <div>
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
                <input type="hidden" name="entry[author]" value="662008882a23e829e3ef6351" />
                <div class="input-item">
                    <input type="submit" />
                </div>
            </div>
        </ActionForm>
    }
}
