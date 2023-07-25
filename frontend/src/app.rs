#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::{use_init_atom_root, AtomRef};

use crate::{
    components::{post::PostManager, toaster::ToastRoot, toaster::Toaster, Navbar, local_profile::LocalProfile},
    prelude::*,
};

// NOTE `static` keyword is used to give a 'static lifetime to the variable
pub static TOASTER: AtomRef<Toaster> = |_| Toaster::default();
pub static POST_MANAGER: AtomRef<PostManager> = |_| PostManager::default();
pub static LOCAL_PROFILE: AtomRef<LocalProfile> = |_| LocalProfile::default();

pub fn Init(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let toaster = use_toaster(cx);
    let local_profile = use_local_profile(cx);


    let _fetch_local_profile = {
        to_owned![api_client, toaster, router, local_profile];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::user::endpoint::{GetMyProfile, GetMyProfileOk};
            let response = fetch_json!(<GetMyProfileOk>, api_client, GetMyProfile);
            match response {
                Ok(res) => {
                    local_profile.write().image = res.profile_image;
                    local_profile.write().user_id = Some(res.user_id);
                },
                Err(_) => {
                    
                    toasty!(toaster => error: format!("Please log in or create an account to continue."));
                router.navigate_to(page::ACCOUNT_LOGIN);
                }
            }
        })
    };

    None
}

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let toaster = use_toaster(cx);

    // NOTE `cx.render` renders the result of the rsx! macro
    // ? The rsx! macro is a macro that returns an `Element` type
    cx.render(rsx! {
        Router { 
            Init {}
            main { class: "max-w-[var(--content-max-width)] min-w-[var(--content-min-width)] mt-[var(--appbar-height)] mb-[var(--navbar-height)] mx-auto p-4",
                Route { to: page::HOME, page::Home {} }
                Route { to: page::HOME_LIKED, page::Liked {} }
                Route { to: page::HOME_BOOKMARKED, page::Bookmarked {} }
                Route { to: page::ACCOUNT_REGISTER, page::Register {} }
                Route { to: page::ACCOUNT_LOGIN, page::Login {} }
                Route { to: page::POST_NEW_CHAT, page::NewChat {} }
                Route { to: page::POST_NEW_IMAGE, page::NewImage {} }
                Route { to: page::POST_NEW_POLL, page::NewPoll {} }
                Route { to: page::POSTS_TRENDING, page::Trending {} }
                Route { to: page::PROFILE_EDIT, page::EditProfile {} }
                Route { to: page::PROFILE_VIEW, page::ViewProfile {} }
            }
            ToastRoot { toaster: toaster }
            Navbar {}
        }
    })
}
