#![allow(non_snake_case)]

use crate::{
    components::post::{use_post_manager, PublicPostEntry},
    prelude::*,
};

use api_client::ApiClient;
use dioxus::{html::h1, prelude::*};
use dioxus_router::use_router;
use toaster::use_toaster;
use uchat_domain::{self, UserFacingError};

use crate::{components::toaster, toasty, util::api_client};

pub fn Trending(cx: Scope) -> Element {
    let api_client = ApiClient::global();

    let router = use_router(cx);
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);

    let _fetch_trending_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{TrendingPosts, TrendingPostsOk};
            toasty!(toaster => info: "Retrieving trending posts...", 3);
            let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        })
    };

    let trending_posts = post_manager
        .read()
        .posts
        .iter()
        .map(|(&id, _)| {
            rsx! {
                div {
                    PublicPostEntry {
                        post_id: id
                    }
                }
            }
        })
        .collect::<Vec<LazyNodes>>();

    cx.render(rsx! {
        trending_posts.into_iter()
    })
}
