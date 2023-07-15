#![allow(non_snake_case)]

use crate::components::post::{use_post_manager, PublicPostEntry};
use crate::components::toaster::use_toaster;
use crate::prelude::*;
use chrono::Duration;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(cx);
    let router = use_router(cx);

    let _fetch_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{HomePosts, HomePostsOk};
            let response = fetch_json!(<HomePostsOk>, api_client, HomePosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        })
    };

    let posts_el = post_manager.read().to_public_posts();

    cx.render(rsx! {
        AppBar { title: "Home",
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::HOME_LIKED, None, None),
                img: "/static/icons/icon-like.svg",
                label: "Liked",
                title: "Show liked posts",
            }
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::HOME_BOOKMARKED, None, None),
                img: "/static/icons/icon-bookmark.svg",
                label: "Saved",
                title: "Show bookmarked posts",
            }
            AppBarImgButton {
                click_handler: move |_| router.replace_route(page::HOME, None, None),
                img: "/static/icons/icon-home.svg",
                label: "Home",
                title: "Go to home page",
                disabled: true,
                append_class: app_bar::BUTTON_SELECTED,
            }
        }
        posts_el.into_iter()
    })
}
