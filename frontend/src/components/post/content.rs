#![allow(non_snake_case)]

use std::collections::HashSet;

use crate::{prelude::*, toasty};
use dioxus::prelude::*;
use itertools::Itertools;
use uchat_domain::ids::{PollChoiceId, PostId};
use uchat_endpoint::post::types::{
    Chat as EndpointChat, Content as EndpointContent, Image as EndpointImage, ImageKind,
    Poll as EndpointPoll, PublicPost, VoteCast,
};

#[inline_props]
pub fn Image<'a>(cx: Scope<'a>, post_id: PostId, content: &'a EndpointImage) -> Element {
    let url = if let ImageKind::Url(url) = &content.kind {
        url
    } else {
        return cx.render(rsx! {"image not found"});
    };

    let caption_el = content.caption.as_ref().map(|caption| {
        rsx! {
            figcaption { em { "{caption.as_ref()}" } }
        }
    });

    cx.render(rsx! {
        figure { class: "flex flex-col gap-2",
            caption_el,
            img { class: "w-full object-contain max-h-[80vh]", src: "{url}" }
        }
    })
}

#[inline_props]
pub fn Chat<'a>(cx: Scope<'a>, post_id: PostId, content: &'a EndpointChat) -> Element {
    let headline_el = content.headline.as_ref().map(|headline| {
        rsx! { div { class: "font-bold", "{headline.as_ref()}" } }
    });

    cx.render(rsx! {
        div { headline_el }
    })
}

#[inline_props]
pub fn Poll<'a>(cx: Scope<'a>, post_id: PostId, content: &'a EndpointPoll) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let vote_onclick = async_handler!(
        &cx,
        [api_client, toaster],
        move |post_id, choice_id| async move {
            use uchat_endpoint::post::endpoint::{Vote, VoteOk};
            let request = Vote { post_id, choice_id };
            match fetch_json!(<VoteOk>, api_client, request) {
                Ok(res) => match res.cast {
                    VoteCast::Yes => toasty!(toaster => success: "Vote cast!", 3),
                    VoteCast::AlreadyVoted => toasty!(toaster => info: "Already voted"),
                },
                Err(e) => toasty!(toaster => error: format!("Failed to cast vote: {e}")),
            }
        }
    );

    let total_votes = content
        .choices
        .iter()
        .map(|choice| choice.num_votes)
        .sum::<i64>();

    let leader_ids = {
        let leaders = content
            .choices
            .iter()
            .max_set_by(|x, y| x.num_votes.cmp(&y.num_votes));
        let ids: HashSet<PollChoiceId> = HashSet::from_iter(leaders.iter().map(|choice| choice.id));
        ids
    };

    // TODO update vote bar on click instead of on page refresh
    let choices_el = content.choices.iter().map(|choice| {
        let percent = if total_votes > 0 {
            let percent = (choice.num_votes as f64 / total_votes as f64) * 100.0;
            format!("{percent:.0}%")
        } else {
            "0%".to_owned()
        };
        let is_leader = leader_ids.contains(&choice.id);

        let bg_color = if is_leader {
            "bg-blue-300"
        } else {
            "bg-neutral-300"
        };

        let foreground_styles = maybe_class!("font-bold", is_leader);

        rsx! {
            li {
                key: "{choice.id.to_string()}",
                class: "relative p-2 m-2 cursor-pointer grid grid-cols-[3rem_1fr] border rounded border-slate-400",
                onclick: move |_| vote_onclick(*post_id, choice.id),
                div {
                    class: "absolute left-0 {bg_color} h-full rounded z-[-1]",
                    style: "width: {percent}"
                }
                div { class: "{foreground_styles}", "{percent}" }
                div { class: "{foreground_styles}", "{choice.description.as_ref()}" }
            }
        }
    });

    let headline_el = rsx! { figcaption { "{content.headline.as_ref()}" } };

    cx.render(rsx! {
        div {
            headline_el,
            ul { choices_el.into_iter() }
        }
    })
}

#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element {
    cx.render(rsx! {
        div {
            match &post.content {
                EndpointContent::Chat(content) => rsx! {Chat { post_id: post.id, content: content }},
                EndpointContent::Image(content) => rsx! {Image { post_id: post.id, content: content }},
                EndpointContent::Poll(content) => rsx! {Poll { post_id: post.id, content: content }},
            }
        }
    })
}
