use leptos::{*, html::Input};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::js_sys;
use web_sys::HtmlInputElement;

#[component]
pub fn FileUpload(ssf: WriteSignal<Option<String>>) -> impl IntoView {
    let file_input = create_node_ref::<Input>();
    let on_file_change = move |_| {
        if let Some(files) = file_input.get().and_then(|f| f.dyn_ref::<HtmlInputElement>().unwrap().files()) {
            let file = files.get(0).unwrap();
            let file_blob_promise = js_sys::Promise::resolve(&file.array_buffer());

            spawn_local(async move {
                let bytes = wasm_bindgen_futures::JsFuture::from(file_blob_promise).await.unwrap();
                let byte_arr: Vec<u8> = js_sys::Uint8Array::new(&bytes).to_vec();
                let s = serde_json::to_string(&byte_arr).unwrap();
                ssf.set(Some(s));
            });
        }
    };

    view! {
        <input ref=file_input type="file" on:change=on_file_change/>
    }
}

