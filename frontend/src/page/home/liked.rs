#![allow(non_snake_case)]

use super::HomePages;
use crate::{components::post::use_post_manager, page::home::home_app_bar::HomeAppBar, prelude::*};
use dioxus::prelude::*;

pub fn Liked(cx: Scope) -> Element {
    let post_manager = use_post_manager(cx);
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);
    let router = use_router(cx);

    let _fetch_posts = {
        to_owned![post_manager, toaster, api_client];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{LikedPosts, LikedPostsOk};

            post_manager.write().clear();
            let res = fetch_json!(<LikedPostsOk>, api_client, LikedPosts);
            match res {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        });
    };

    // let posts_el = post_manager.read().to_public_posts();
    // TODO refactor common pattern in Home, Bookmarked and Liked pages
    let posts_el = {
        let posts = post_manager.read().to_public_posts();
        if posts.is_empty() {
            let trending_link_el = rsx! {
                a {
                    class: "link",
                    onclick: move |_| {
                        router.navigate_to(page::POSTS_TRENDING);
                    },
                    "trending"
                }
            };
            rsx! {
                div { class: "flex flex-col text-center justify-center h-[calc(100vh_-_var(--navbar-height)_-_var(--appbar-height))]",
                    span {
                        "You don't have any liked posts. Check out what's "
                        trending_link_el,
                        ", and follow some users to get started."
                    }
                }
            }
        } else {
            rsx! {posts.into_iter()}
        }
    };

    cx.render(rsx! {
        HomeAppBar { title: "Liked Posts".to_owned(), active_page: HomePages::Liked }
        posts_el
    })
}
