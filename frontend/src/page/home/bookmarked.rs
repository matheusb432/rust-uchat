#![allow(non_snake_case)]

use super::HomePages;
use crate::{components::post::use_post_manager, page::home::home_app_bar::HomeAppBar, prelude::*};
use dioxus::prelude::*;

pub fn Bookmarked(cx: Scope) -> Element {
    let post_manager = use_post_manager(cx);
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);

    let _fetch_posts = {
        to_owned![post_manager, toaster, api_client];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{BookmarkedPosts, BookmarkedPostsOk};

            let res = fetch_json!(<BookmarkedPostsOk>, api_client, BookmarkedPosts);
            match res {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        });
    };

    let posts_el = post_manager.read().to_public_posts();

    cx.render(rsx! {
        HomeAppBar {
            title: "Bookmarked Posts".to_owned(),
            active_page: HomePages::Bookmarked
        }
        posts_el.into_iter()
    })
}
