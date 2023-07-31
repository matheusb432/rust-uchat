#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

use super::use_post_manager;

#[inline_props]
pub fn PostsList(cx: Scope, empty_message: String) -> Element {
    let post_manager = use_post_manager(cx);
    let router = use_router(cx);

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
                        "{empty_message}. Check out what's "
                        trending_link_el,
                        ", and follow some users to get started."
                    }
                }
            }
        } else {
            rsx! {posts.into_iter()}
        }
    };

    cx.render(rsx! {posts_el})
}
