#![allow(non_snake_case)]

use crate::{components::keyed_notification_box::KeyedNotifications, prelude::*, util};
use dioxus::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone, Debug)]
enum PreviewImageData {
    DataUrl(String),
    Remote(String),
}

#[derive(Clone, Debug, Default)]
pub struct PageState {
    form_errors: KeyedNotifications,

    display_name: String,
    email: String,
    password: String,
    password_confirm: String,
    profile_image: Option<PreviewImageData>,
}

// TODO Refactor ImageInput and ImagePreview to reuse code
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
                Ok(data) => page_state
                    .with_mut(|state| state.profile_image = Some(PreviewImageData::DataUrl(data))),
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
    let image_data = page_state.with(|state| state.profile_image.clone());

    let img_el = |img_src| {
        rsx! {
            img {
                class: "profile-portrait-lg",
                src: "{img_src}"
            }
        }
    };

    let image_data = match image_data {
        Some(PreviewImageData::DataUrl(ref data)) => img_el(data),
        Some(PreviewImageData::Remote(ref url)) => img_el(url),
        None => rsx! { "" },
    };

    cx.render(rsx! { image_data })
}

pub fn EditProfile(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);
    let router = use_router(cx);

    cx.render(rsx! {
       form {
           class: "flex flex-col w-full gap-3",
           prevent_default: "onsubmit",

           ImagePreview { page_state: page_state.clone() },
           ImageInput { page_state: page_state.clone() },

           div {
               class: "flex justify-end gap-3",
               button {
                   class: "btn",
                   prevent_default: "onclick",
                   onclick: move |_| router.pop_route(),
                   "Cancel"
               }
               button {
                   class: "btn",
                   r#type: "submit",
                   onclick: move |_| router.pop_route(),
                   "Save"
               }
           }
       }
    })
}
