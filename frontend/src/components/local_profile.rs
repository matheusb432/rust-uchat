#![allow(non_snake_case)]

use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};
use uchat_domain::ids::UserId;
use url::Url;

#[derive(Default)]
pub struct LocalProfile {
    pub image: Option<Url>,
    pub user_id: Option<UserId>,
}

pub fn use_local_profile(cx: &ScopeState) -> &UseAtomRef<LocalProfile> {
    use_atom_ref(cx, crate::app::LOCAL_PROFILE)
}
