#![allow(non_snake_case)]

use crate::{prelude::*, toasty, util::api_client};
use dioxus::prelude::*;
use uchat_domain::ids::PostId;

use super::use_post_manager;

#[inline_props]
pub fn Bookmark(cx: Scope, post_id: PostId, bookmarked: bool) -> Element {
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let bookmark_onclick = async_handler!(
        &cx,
        [api_client, post_manager, toaster, post_id],
        move |_| async move {
            use uchat_endpoint::post::endpoint::{Bookmark, BookmarkOk};
            use uchat_endpoint::post::types::BookmarkAction;

            let action = match post_manager.read().get(&post_id).unwrap().bookmarked {
                true => BookmarkAction::Remove,
                false => BookmarkAction::Add,
            };

            let request = Bookmark { action, post_id };
            match fetch_json!(<BookmarkOk>, api_client) {
                Ok(res) => {
                    post_manager.write().update(post_id, |post| {
                        post.bookmarked = res.status.into();
                    });
                }
                Err(e) => toasty!(toaster => error: format!("Failed to bookmark post: {e}")),
            }
        }
    );

    let icon = if bookmarked {
        "/static/icons/icon-bookmark-saved.svg"
    } else {
        "/static/icons/icon-bookmark.svg"
    };

    cx.render(rsx! {
        div {
            class: "cursor-pointer",
            onclick: bookmark_onclick,
            img {
                class: "actionbar-icon",
                src: "{icon}",
            }
        }
    })
}

#[inline_props]
pub fn ActionBar(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);

    let this_post = post_manager.read();
    let this_post = this_post.get(&post_id).unwrap();
    let this_post_id = this_post.id;

    cx.render(rsx! {
        div {
            class: "flex justify-between w-full opacity-70 mt-4",
            // boost
            // bookmark
            // like & dislike
            // comment
        }
        // quick respond
    })
}
