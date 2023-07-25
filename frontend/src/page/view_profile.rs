#![allow(non_snake_case)]

use std::str::FromStr;

use crate::{components::post::use_post_manager, prelude::*};
use chrono::Utc;
use dioxus::prelude::*;

use toaster::use_toaster;
use uchat_domain::ids::UserId;

use crate::{components::toaster, toasty};

use super::edit_profile::PreviewImageData;

#[derive(PartialEq, Props)]
pub struct ImagePreviewProps {
    #[props(!optional)]
    image_data: Option<PreviewImageData>,
}

// TODO refactor
pub fn ImagePreview(cx: Scope<'_, ImagePreviewProps>) -> Element<'_> {
    let img_el = |img_src| {
        rsx! {
            img {
                class: "profile-portrait-lg",
                src: "{img_src}"
            }
        }
    };

    let image_data = match cx.props.image_data {
        Some(PreviewImageData::DataUrl(ref data)) => img_el(data),
        Some(PreviewImageData::Remote(ref url)) => img_el(url),
        None => rsx! { "" },
    };

    cx.render(rsx! { image_data })
}

#[derive(Default)]
pub struct ViewProfileData {
    pub display_name: Option<String>,
    pub handle: String,
    pub profile_image: Option<PreviewImageData>,
}

pub fn ViewProfile(cx: Scope) -> Element {
    let route = use_route(cx);
    let user_id = route
        .segment("id")
        .map(|id| UserId::from_str(id).ok())
        .flatten();

    let Some(user_id) = user_id else {
        return cx.render(rsx! {
            "Post not found!"
        });
    };

    let api_client = ApiClient::global();

    let router = use_router(cx);
    let post_manager = use_post_manager(cx);
    let profile = use_state(cx, ViewProfileData::default);
    let is_following = use_state(cx, || false);
    let toaster = use_toaster(cx);

    let _fetch_profile = {
        to_owned![
            api_client,
            toaster,
            post_manager,
            user_id,
            profile,
            is_following
        ];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::user::endpoint::{
                IsFollowing, IsFollowingOk, ViewProfile, ViewProfileOk,
            };

            let view_res =
                fetch_json!(<ViewProfileOk>, api_client, ViewProfile { user_id: user_id });
            let is_following_res =
                fetch_json!(<IsFollowingOk>, api_client, IsFollowing { follows: user_id });
            match (view_res, is_following_res) {
                (Ok(view_res), Ok(is_following_res)) => {
                    post_manager.write().populate(view_res.posts.into_iter());
                    is_following.set(is_following_res.is_following);
                    profile.set(ViewProfileData {
                        display_name: view_res.display_name,
                        handle: view_res.handle,
                        profile_image: view_res
                            .profile_image
                            .map(|img| PreviewImageData::Remote(img.to_string())),
                    });
                }
                _ => {
                    toasty!(toaster => error: format!("Failed to retrieve profile"))
                }
            };
        })
    };

    let follow_onclick = async_handler!(
        &cx,
        [api_client, user_id, is_following, toaster],
        move |_| async move {
            use uchat_endpoint::user::endpoint::{Follow, FollowOk};
            use uchat_endpoint::user::types::FollowAction;

            let target_action = if *is_following.get() {
                FollowAction::Unfollow
            } else {
                FollowAction::Follow
            };
            let request = Follow {
                follows: user_id,
                action: target_action,
            };

            let response = fetch_json!(<FollowOk>, api_client, request);

            match response {
                Ok(res) => match res.is_following {
                    true => {
                        toasty!(toaster => success: "successfully followed!", 3);
                        is_following.set(true);
                    }
                    false => {
                        toasty!(toaster => success: "successfully unfollowed!", 3);
                        is_following.set(false);
                    }
                },
                Err(e) => toasty!(toaster => error: format!("Failed to update follow status: {e}")),
            }
        }
    );

    let posts_el = post_manager.read().to_public_posts();
    let display_name_el = match profile.get().display_name.clone() {
        Some(name) => rsx! {name},
        None => rsx! {""},
    };
    let follow_btn_label = if *is_following.get() {
        "Following"
    } else {
        "Follow"
    };

    cx.render(rsx! {
        AppBar {
            title: "View Profile",
            AppBarImgButton {
                click_handler: move |_| router.pop_route(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page"
            }
        }
        section {
            class: "flex flex-col items-center justify-center",
            ImagePreview { image_data: profile.get().profile_image.clone() }
            display_name_el
            span {
                "@{profile.get().handle.clone()}"
            }
        }
        section {
            class: "flex gap-x-6 items-center justify-center mt-6 mb-8",
            button {
                class: "btn",
                prevent_default: "onclick",
                // TODO should open post with `direct_message_to` set
                onclick: move |_| router.pop_route(),
                "Send Message"
            }
            button {
                class: "btn",
                prevent_default: "onclick",
                onclick: follow_onclick,
                follow_btn_label
            }
        }
        posts_el.into_iter()
    })
}
