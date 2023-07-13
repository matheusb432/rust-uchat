#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::{use_init_atom_root, AtomRef};

use crate::{
    components::{post::PostManager, toaster::ToastRoot, toaster::Toaster, Navbar},
    prelude::*,
};

// NOTE `static` keyword is used to give a 'static lifetime to the variable
pub static TOASTER: AtomRef<Toaster> = |_| Toaster::default();
pub static POST_MANAGER: AtomRef<PostManager> = |_| PostManager::default();

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let toaster = use_toaster(cx);

    // NOTE `cx.render` renders the result of the rsx! macro
    // ? The rsx! macro is a macro that returns an `Element` type
    cx.render(rsx! {
        Router {
            main {
                class: "max-w-[var(--content-max-width)] min-w-[var(--content-min-width)] mb-[var(--navbar-height)] mx-auto p-4",
                Route { to: page::HOME, page::Home {} }
                Route { to: page::ACCOUNT_REGISTER, page::Register {} }
                Route { to: page::ACCOUNT_LOGIN, page::Login {} }
                Route { to: page::POST_NEW_CHAT, page::NewChat {} }
                Route { to: page::POST_NEW_IMAGE, page::NewImage {} }
                Route { to: page::POSTS_TRENDING, page::Trending {} }
            }
            ToastRoot {toaster: toaster}
            Navbar {}
        }
    })
}
