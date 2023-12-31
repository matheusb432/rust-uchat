#![allow(non_snake_case)]

use super::NewPostPages;
use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn NewPostAppBar(cx: Scope, title: String, active_page: NewPostPages) -> Element {
    let router = use_router(cx);
    let is_chat = *active_page == NewPostPages::Chat;
    let is_image = *active_page == NewPostPages::Image;
    let is_poll = *active_page == NewPostPages::Poll;

    cx.render(rsx! {
        AppBar { title: "{title}",
            AppBarImgButton {
                handle_onclick: move |_| router.replace_route(page::POST_NEW_CHAT, None, None),
                img: "/static/icons/icon-messages.svg",
                label: "Chat",
                title: "Post a new chat",
                disabled: is_chat,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_chat)
            }
            AppBarImgButton {
                handle_onclick: move |_| router.replace_route(page::POST_NEW_IMAGE, None, None),
                img: "/static/icons/icon-image.svg",
                label: "Image",
                title: "Post a new image",
                disabled: is_image,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_image)
            }
            AppBarImgButton {
                handle_onclick: move |_| router.replace_route(page::POST_NEW_POLL, None, None),
                img: "/static/icons/icon-poll.svg",
                label: "Poll",
                title: "Post a new poll",
                disabled: is_poll,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_poll)
            }
            AppBarImgButton {
                handle_onclick: move |_| router.pop_route(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page"
            }
        }
    })
}
