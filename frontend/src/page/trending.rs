#![allow(non_snake_case)]

use crate::prelude::*;

use api_client::ApiClient;
use dioxus::{html::h1, prelude::*};
use dioxus_router::use_router;
use toaster::use_toaster;
use uchat_domain::{self, UserFacingError};

use crate::{components::toaster, toasty, util::api_client};

pub fn Trending(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let toaster = use_toaster(cx);

    let _fetch_trending_posts = {
        to_owned![api_client, toaster];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{TrendingPosts, TrendingPostsOk};
            toasty!(toaster => info: "Retrieving trending posts...", 3);
            let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
            match response {
                Ok(res) => (),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        })
    };

    cx.render(rsx! {
        h1 { "Trending" }
    })
}
