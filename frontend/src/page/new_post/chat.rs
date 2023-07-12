#![allow(non_snake_case)]

use crate::{fetch_json, prelude::*, toasty, util::api_client};
use chrono::Duration;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_endpoint::post::types::NewPostOptions;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub message: String,
    pub headline: String,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        use uchat_domain::post::{Headline, Message};

        if Message::new(&self.message).is_err() {
            return false;
        }

        if !self.headline.is_empty() && Headline::new(&self.headline).is_err() {
            return false;
        }
        true
    }
}

#[inline_props]
pub fn MessageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::Message;

    let max_chars = Message::MAX_CHARS;
    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().message.len() > max_chars || page_state.read().message.is_empty()
    );
    cx.render(rsx! {
        div {
            label { r#for: "message",
                div { class: "flex flex-row justify-between",
                    span { "Message" }
                    span { class: "text-right {wrong_len}", "{page_state.read().message.len()}/{max_chars}" }
                }
            }
            textarea {
                class: "input-field",
                id: "message",
                rows: 5,
                value: "{page_state.read().message}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.message = ev.data.value.clone());
                }
            }
        }
    })
}

#[inline_props]
pub fn HeadlineInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::Headline;

    let max_chars = Headline::MAX_CHARS;
    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().headline.len() > max_chars
    );

    cx.render(rsx! {
        div {
            label { r#for: "headline",
                div { class: "flex flex-row justify-between",
                    span { "Headline" }
                    span { class: "text-right {wrong_len}", "{page_state.read().headline.len()}/{max_chars}" }
                }
            }
            input {
                class: "input-field",
                id: "headline",
                value: "{page_state.read().headline}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.headline = ev.data.value.clone());
                }
            }
        }
    })
}

pub fn NewChat(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);
    let is_invalid = !page_state.read().can_submit();
    let toaster = use_toaster(cx);
    let router = use_router(&cx);
    let api_client = ApiClient::global();

    let form_onsubmit = async_handler!(
        &cx,
        [toaster, api_client, page_state, router],
        move |_| async move {
            use uchat_domain::post::{Headline, Message};
            use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
            use uchat_endpoint::post::types::Chat;

            let request = NewPost {
                content: Chat {
                    headline: {
                        let headline = &page_state.read().headline;
                        // TODO refactor (ok() should do it? also two reads on the page_state?)
                        if headline.is_empty() {
                            None
                        } else {
                            Some(Headline::new(headline).unwrap())
                        }
                    },
                    message: Message::new(&page_state.read().message).unwrap(),
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

    let submit_btn_style = maybe_class!("btn-disabled", is_invalid);

    cx.render(rsx! {
        form { class: "flex flex-col gap-4", onsubmit: form_onsubmit, prevent_default: "onsubmit",
            MessageInput { page_state: page_state.clone() }
            HeadlineInput { page_state: page_state.clone() }
            button { class: "btn {submit_btn_style}", r#type: "submit", disabled: is_invalid, "Post" }
        }
    })
}
