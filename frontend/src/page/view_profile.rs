#![allow(non_snake_case)]

use std::str::FromStr;

use crate::{components::post::use_post_manager, prelude::*};
use dioxus::prelude::{GlobalAttributes, *};

use uchat_domain::ids::UserId;
use uchat_endpoint::user::types::FollowAction;

use crate::toasty;

use super::edit_profile::PreviewImageData;

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
        .and_then(|id| UserId::from_str(id).ok());

    let Some(user_id) = user_id else {
        return cx.render(rsx! {
            "Post not found!"
        });
    };

    let api_client = ApiClient::global();

    let router = use_router(cx);
    let post_manager = use_post_manager(cx);
    let local_profile = use_local_profile(cx);
    let profile = use_ref(cx, || None);
    let toaster = use_toaster(cx);

    use_effect(cx, (&user_id,), |(user_id,)| {
        to_owned![api_client, toaster, post_manager, user_id, profile];
        async move {
            use uchat_endpoint::user::endpoint::{ViewProfile, ViewProfileOk};

            post_manager.write().clear();
            let view_res =
                fetch_json!(<ViewProfileOk>, api_client, ViewProfile { user_id });
            match view_res {
                Ok(view_res) => {
                    post_manager.write().populate(view_res.posts.into_iter());
                    profile.with_mut(|profile| *profile = Some(view_res.profile));
                }
                Err(e) => {
                    toasty!(toaster => error: format!("Failed to retrieve profile: {e}"))
                }
            };
        }
    });

    let follow_onclick = async_handler!(
        &cx,
        [api_client, user_id, profile, toaster],
        move |_| async move {
            use uchat_endpoint::user::endpoint::{FollowUser, FollowUserOk};

            let am_following = match profile.read().as_ref() {
                Some(p) => p.am_following,
                None => false,
            };

            let request = FollowUser {
                follows: user_id,
                action: if am_following {
                    FollowAction::Unfollow
                } else {
                    FollowAction::Follow
                },
            };

            let response = fetch_json!(<FollowUserOk>, api_client, request);

            match response {
                Ok(res) => {
                    let am_following: bool = res.is_following;
                    match am_following {
                        true => {
                            toasty!(toaster => success: "successfully followed!", 3);
                        }
                        false => {
                            toasty!(toaster => success: "successfully unfollowed!", 3);
                        }
                    }
                    profile.with_mut(|profile| {
                        profile.as_mut().map(|p| p.am_following = am_following)
                    });
                }
                Err(e) => toasty!(toaster => error: format!("Failed to update follow status: {e}")),
            }
        }
    );

    let posts_el = post_manager.read().to_public_posts();

    let profile_el = match profile.with(|profile| profile.clone()) {
        Some(p) => {
            let follow_btn_label = if p.am_following { "Unfollow" } else { "Follow" };
            let display_name_el = match p.display_name {
                Some(name) => rsx! {"Name: {name.into_inner()}"},
                None => rsx! {""},
            };
            let profile_image = p
                .profile_image
                .map(|url| url.to_string())
                .unwrap_or_else(|| "".to_string());
            let user_btns = local_profile.read().user_id.map(|id| {
                if id == p.id {
                    rsx! {""}
                } else {
                    rsx! {
                        Button { r#type: BtnTypes::Button, handle_onclick: || router.pop_route(), "Send Message" }
                        Button { r#type: BtnTypes::Button, handle_onclick: move || follow_onclick(()), follow_btn_label }
                    }
                }
            });

            rsx! {
                section { class: "flex flex-col items-center justify-center gap-3",
                    div { class: "flex flex-row justify-center", img { class: "profile-portrait-lg", src: "{profile_image}" } }
                    display_name_el,
                    span { "Handle: @{p.handle.clone()}" }
                }
                section { class: "flex gap-x-6 items-center justify-center mt-6 mb-8", user_btns }
            }
        }
        None => rsx! {"Loading..."},
    };

    cx.render(rsx! {
        AppBar { title: "View Profile",
            AppBarImgButton {
                handle_onclick: move |_| router.pop_route(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page"
            }
        }
        profile_el,
        div { class: "font-bold text-center my-6", "Posts" }
        hr { class: "h-px my-6 bg-gray-200 border-0" }
        posts_el.into_iter()
    })
}
