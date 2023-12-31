#![allow(non_snake_case)]

use crate::{
    fetch_json,
    page::new_post_app_bar::NewPostAppBar,
    prelude::*,
    ret_if, toasty,
    util::{self},
};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_endpoint::post::types::{ImageKind, NewPostOptions};
use web_sys::HtmlInputElement;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub caption: String,
    pub image: Option<String>,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        use uchat_domain::post::Caption;

        ret_if!(
            !self.caption.is_empty() && Caption::new(&self.caption).is_err(),
            false
        );
        ret_if!(self.image.is_none(), false);

        true
    }
}

#[inline_props]
pub fn ImageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let toaster = use_toaster(cx);
    let handle_oninput = |_| {
        to_owned![page_state, toaster];
        async move {
            use gloo_file::{futures::read_as_data_url, File};
            use wasm_bindgen::JsCast;

            let el = util::document()
                .get_element_by_id("image-input")
                .unwrap()
                .unchecked_into::<HtmlInputElement>();
            let file: File = el.files().unwrap().get(0).unwrap().into();

            match read_as_data_url(&file).await {
                Ok(data) => page_state.with_mut(|state| state.image = Some(data)),
                Err(e) => {
                    toasty!(toaster => error: format!("Error loading file: {e}"));
                }
            }
        }
    };

    cx.render(rsx! {
        div {
            label { r#for: "image-input", "Upload Image" }
            input {
                class: "w-full",
                id: "image-input",
                r#type: "file",
                accept: "image/*",
                oninput: handle_oninput
            }
        }
    })
}

#[inline_props]
pub fn ImagePreview(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let image_data = page_state.read().image.clone();
    let preview_el = if let Some(ref image) = image_data {
        rsx! {img {
            class: "max-w-[calc(var(--content-max-width)/2)] max-h-[40vh]",
            src: "{image}"
        }
        }
    } else {
        rsx! { div { "no image uploaded" } }
    };

    cx.render(rsx! {preview_el})
}

#[inline_props]
pub fn CaptionInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::Caption;

    let max_chars = Caption::MAX_CHARS;
    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().caption.len() > max_chars
    );

    cx.render(rsx! {
        div {
            label { r#for: "caption",
                div { class: "flex flex-row justify-between",
                    span { "Caption" }
                    span { class: "text-right {wrong_len}", "{page_state.read().caption.len()}/{max_chars}" }
                }
            }
            input {
                class: "input-field",
                id: "caption",
                value: "{page_state.read().caption}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.caption = ev.data.value.clone());
                }
            }
        }
    })
}

pub fn NewImage(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);
    let is_invalid = !page_state.read().can_submit();
    let toaster = use_toaster(cx);
    let router = use_router(cx);
    let api_client = ApiClient::global();

    let form_onsubmit = async_handler!(
        &cx,
        [toaster, api_client, page_state, router],
        move |_| async move {
            use uchat_domain::post::Caption;
            use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
            use uchat_endpoint::post::types::Image;

            let read_ps = &page_state.read();

            let request = NewPost {
                content: Image {
                    caption: {
                        let caption = &read_ps.caption;
                        if caption.is_empty() {
                            None
                        } else {
                            Caption::new(caption).ok()
                        }
                    },
                    kind: {
                        let image = &read_ps.image;
                        ImageKind::DataUrl(image.clone().unwrap())
                    },
                }
                .into(),
                options: NewPostOptions::default(),
            };

            let response = fetch_json!(<NewPostOk>, api_client, request);
            match response {
                Ok(_) => {
                    router.replace_route(page::HOME, None, None);
                    toasty!(toaster => success: "new post created!", 3);
                }
                Err(e) => {
                    toasty!(toaster => error: format!("Post failed: {e}"));
                }
            }
        }
    );

    cx.render(rsx! {
        NewPostAppBar { title: "New Image".to_owned(), active_page: super::NewPostPages::Image }
        form { class: "flex flex-col gap-4", onsubmit: form_onsubmit, prevent_default: "onsubmit",
            ImageInput { page_state: page_state.clone() }
            ImagePreview { page_state: page_state.clone() }
            CaptionInput { page_state: page_state.clone() }
            Button::<fn()> { r#type: BtnTypes::Submit, disabled: is_invalid, "Post" }
        }
    })
}
