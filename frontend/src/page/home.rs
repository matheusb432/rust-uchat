#![allow(non_snake_case)]

pub mod bookmarked;
pub mod liked;

pub mod home_app_bar;

pub use bookmarked::Bookmarked;
pub use liked::Liked;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HomePages {
    Liked,
    Bookmarked,
    Home,
}

use crate::components::post::use_post_manager;
use crate::components::toaster::use_toaster;
use crate::page::home::home_app_bar::HomeAppBar;
use crate::prelude::*;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let post_manager = use_post_manager(cx);

    let _fetch_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{HomePosts, HomePostsOk};

            post_manager.write().clear();
            let response = fetch_json!(<HomePostsOk>, api_client, HomePosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        })
    };

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
                div {
                    class: "flex flex-col text-center justify-center h-[calc(100vh_-_var(--navbar-height)_-_var(--appbar-height))]",
                    span {
                        "Check out what's ", trending_link_el, ", and follow some users to get started."
                    }
                }
            }
        } else {
            rsx! {
                posts.into_iter()
            }
        }
    };

    cx.render(rsx! {
        HomeAppBar { title: "Home".to_owned(), active_page: HomePages::Home }
        posts_el
    })
}
