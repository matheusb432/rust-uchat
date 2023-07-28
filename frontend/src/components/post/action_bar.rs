#![allow(non_snake_case)]

use crate::{components::post::quick_respond::QuickRespond, prelude::*, toasty};
use dioxus::prelude::*;
use uchat_domain::ids::PostId;
use uchat_endpoint::post::types::LikeStatus;

use super::use_post_manager;

#[derive(Props)]
struct ActionBarIconProps<'a, F>
where
    F: Fn(Event<MouseData>),
{
    icon: &'a str,
    #[props(into)]
    label: &'a str,
    handle_onclick: F,
}

fn ActionBarIcon<'a, F>(cx: Scope<'a, ActionBarIconProps<'a, F>>) -> Element<'a>
where
    F: Fn(Event<MouseData>),
{
    cx.render(rsx! {
        div {
            class: "cursor-pointer",
            onclick: move |ev| {
                (cx.props.handle_onclick)(ev);
            },
            img { class: "actionbar-icon", src: "{cx.props.icon}" }
            div { class: "actionbar-icon-text", "{cx.props.label}" }
        }
    })
}

#[inline_props]
pub fn LikeDislike(
    cx: Scope,
    post_id: PostId,
    like_status: LikeStatus,
    likes: i64,
    dislikes: i64,
) -> Element {
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let like_icon = match like_status {
        LikeStatus::Like => "/static/icons/icon-like-selected.svg",
        _ => "/static/icons/icon-like.svg",
    };

    let dislike_icon = match like_status {
        LikeStatus::Dislike => "/static/icons/icon-dislike-selected.svg",
        _ => "/static/icons/icon-dislike.svg",
    };

    let like_onclick = async_handler!(
        &cx,
        [api_client, post_manager, toaster, post_id],
        move |like_status| async move {
            use uchat_endpoint::post::endpoint::{React, ReactOk};
            let like_status = {
                if post_manager.read().get(&post_id).unwrap().like_status == like_status {
                    LikeStatus::NoReaction
                } else {
                    like_status
                }
            };

            let request = React {
                like_status,
                post_id,
            };
            match fetch_json!(<ReactOk>, api_client, request) {
                Ok(res) => {
                    post_manager.write().update(post_id, |post| {
                        post.like_status = res.like_status;
                        post.likes = res.likes;
                        post.dislikes = res.dislikes;
                    });
                }
                Err(e) => toasty!(toaster => error: format!("Failed to react to post: {e}")),
            }
        }
    );

    cx.render(rsx! {
        ActionBarIcon {
            icon: like_icon,
            label: "{likes}",
            handle_onclick: move |_| like_onclick(LikeStatus::Like)
        }
        ActionBarIcon {
            icon: dislike_icon,
            label: "{dislikes}",
            handle_onclick: move |_| like_onclick(LikeStatus::Dislike)
        }
    })
}

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
            match fetch_json!(<BookmarkOk>, api_client, request) {
                Ok(res) => {
                    post_manager.write().update(post_id, |post| {
                        post.bookmarked = res.status.into();
                    });
                }
                Err(e) => toasty!(toaster => error: format!("Failed to bookmark post: {e}")),
            }
        }
    );

    let icon = if *bookmarked {
        "/static/icons/icon-bookmark-saved.svg"
    } else {
        "/static/icons/icon-bookmark.svg"
    };

    cx.render(rsx! {
        div { class: "cursor-pointer", onclick: bookmark_onclick, img { class: "actionbar-icon", src: "{icon}" } }
    })
}

#[inline_props]
pub fn Boost(cx: Scope, post_id: PostId, boosted: bool, boosts: i64) -> Element {
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let boost_onclick = async_handler!(
        &cx,
        [api_client, post_manager, toaster, post_id],
        move |_| async move {
            use uchat_endpoint::post::endpoint::{Boost, BoostOk};
            use uchat_endpoint::post::types::BoostAction;

            let action = match post_manager.read().get(&post_id).unwrap().boosted {
                true => BoostAction::Remove,
                false => BoostAction::Add,
            };

            let request = Boost { action, post_id };
            match fetch_json!(<BoostOk>, api_client, request) {
                Ok(res) => {
                    post_manager.write().update(post_id, |post| {
                        post.boosted = res.status.into();
                        post.boosts += if post.boosted { 1 } else { -1 };
                    });
                }
                Err(e) => toasty!(toaster => error: format!("Failed to boost post: {e}")),
            }
        }
    );

    let icon = if *boosted {
        "/static/icons/icon-boosted.svg"
    } else {
        "/static/icons/icon-boost.svg"
    };

    cx.render(rsx! {
        div { class: "cursor-pointer", onclick: boost_onclick,
            img { class: "actionbar-icon", src: "{icon}" }
            div { class: "text-center", "{boosts}" }
        }
    })
}

#[inline_props]
pub fn Comment(cx: Scope, opened: UseState<bool>) -> Element {
    let comment_onclick = sync_handler!([opened], move |_| {
        let current = *opened.get();
        opened.set(!current);
    });

    cx.render(rsx! {
        div { class: "cursor-pointer", onclick: comment_onclick, img { class: "actionbar-icon", src: "/static/icons/icon-messages.svg" } }
    })
}

#[inline_props]
pub fn QuickRespondBox(cx: Scope, opened: UseState<bool>) -> Element {
    let element = match *opened.get() {
        true => {
            to_owned![opened];
            Some(rsx! { QuickRespond { opened: opened } })
        }
        false => None,
    };

    cx.render(rsx! {element})
}

#[inline_props]
pub fn ActionBar(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);
    let quick_respond_opened = use_state(cx, || false).clone();

    let this_post = post_manager.read();
    let Some(this_post) = this_post.get(post_id) else {
        return None;
    };
    let this_post_id = this_post.id;

    cx.render(rsx! {
        div { class: "flex justify-between w-full opacity-70 mt-4",
            Boost { post_id: this_post_id, boosts: this_post.boosts, boosted: this_post.boosted }
            Bookmark { bookmarked: this_post.bookmarked, post_id: this_post_id }
            LikeDislike {
                post_id: this_post_id,
                likes: this_post.likes,
                dislikes: this_post.dislikes,
                like_status: this_post.like_status
            }
            Comment { opened: quick_respond_opened.clone() }
        }
        QuickRespondBox { opened: quick_respond_opened }
    })
}
