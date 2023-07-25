#![allow(non_snake_case)]

use crate::{
    components::post::{use_post_manager, PublicPostEntry},
    prelude::*,
};

use api_client::ApiClient;
use dioxus::prelude::*;
use toaster::use_toaster;

use crate::{components::toaster, toasty, util::api_client};

pub fn Trending(cx: Scope) -> Element {
    let api_client = ApiClient::global();

    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);

    let _fetch_trending_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{TrendingPosts, TrendingPostsOk};
            let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        })
    };

    let posts_el = post_manager.read().to_public_posts();

    cx.render(rsx! {posts_el.into_iter()})
}
