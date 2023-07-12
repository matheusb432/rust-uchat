#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use uchat_domain::ids::PostId;
use uchat_endpoint::post::types::{Chat as EndpointChat, Content as EndpointContent, PublicPost};

#[inline_props]
pub fn Chat<'a>(cx: Scope<'a>, post_id: PostId, content: &'a EndpointChat) -> Element {
    let headline_el = content.headline.as_ref().map(|headline| {
        rsx! {
            div {
                class: "font-bold",
                "{headline.as_ref()}"
            }
        }
    });

    cx.render(rsx! {
        div {
            headline_el
        }
    })
}

#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element {
    cx.render(rsx! {
        div {
            match &post.content {
                EndpointContent::Chat(content) => rsx! {Chat { post_id: post.id, content: content }}
            }
        }
    })
}
