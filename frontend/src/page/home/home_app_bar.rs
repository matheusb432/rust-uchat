#![allow(non_snake_case)]

use super::HomePages;
use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn HomeAppBar(cx: Scope, title: String, active_page: HomePages) -> Element {
    let router = use_router(cx);
    let is_liked = *active_page == HomePages::Liked;
    let is_bookmarked = *active_page == HomePages::Bookmarked;
    let is_home = *active_page == HomePages::Home;

    cx.render(rsx! {
        AppBar { title: "{title}",
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::HOME_LIKED, None, None),
                img: "/static/icons/icon-like.svg",
                label: "Liked",
                title: "Show liked posts",
                disabled: is_liked,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_liked)
            }
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::HOME_BOOKMARKED, None, None),
                img: "/static/icons/icon-bookmark.svg",
                label: "Saved",
                title: "Show bookmarked posts",
                disabled: is_bookmarked,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_bookmarked)
            }
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::HOME, None, None),
                img: "/static/icons/icon-home.svg",
                label: "Home",
                title: "Go to home page",
                disabled: is_home,
                append_class: maybe_class!(app_bar::BUTTON_SELECTED, is_home)
            }
        }
    })
}
