#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;

use crate::{components::Navbar, prelude::*};

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    // NOTE `cx.render` renders the result of the rsx! macro
    // ? The rsx! macro is a macro that returns an `Element` type
    cx.render(rsx! {
        Router { 
            Route { to: page::HOME, page::Home {} }
            Route { to: page::ACCOUNT_REGISTER, page::Register {} }
            Route { to: page::ACCOUNT_LOGIN, page::Login {} }
            Route { to: page::POST_NEW_CHAT, page::NewChat {} }
            Navbar {}
        }
    })
}
