use leptos::*;
use leptos_router::ActionForm;

use crate::server::AddPost;
use crate::components::utils::FileUpload;

#[component]
pub fn WriteBlog() -> impl IntoView {
    let (sf, ssf) = create_signal(None::<String>);
    let add_post = Action::<AddPost, _>::server();

    view! {
        <FileUpload ssf=ssf />
        <ActionForm action=add_post>
            <input type="text" name="entry[title]" value="" />
            <input type="hidden" name="entry[image]" value=move || sf.get().unwrap_or_default() />
            <input type="submit" />
        </ActionForm>
    }
}
