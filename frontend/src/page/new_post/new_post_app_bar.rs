#![allow(non_snake_case)]

use super::Pages;
use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn NewPostAppBar(cx: Scope, title: String, active_page: Pages) -> Element {
    let router = use_router(cx);
    let is_chat = *active_page == Pages::Chat;
    let is_image = *active_page == Pages::Image;
    let is_poll = *active_page == Pages::Poll;

    cx.render(rsx! {
        AppBar { title: "{title}",
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_CHAT, None, None),
                img: "/static/icons/icon-messages.svg",
                label: "Chat",
                title: "Post a new chat",
                disabled: is_chat,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_chat)
            }
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_IMAGE, None, None),
                img: "/static/icons/icon-image.svg",
                label: "Image",
                title: "Post a new image",
                disabled: is_image,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_image)
            }
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_POLL, None, None),
                img: "/static/icons/icon-poll.svg",
                label: "Poll",
                title: "Post a new poll",
                disabled: is_poll,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_poll)
            }
            AppBarImgButton {
                click_handler: move |_| router.pop_route(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page"
            }
        }
    })
}
