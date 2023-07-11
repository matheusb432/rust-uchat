#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};
use indexmap::IndexMap;
use uchat_domain::ids::PostId;
use uchat_endpoint::post::types::PublicPost;

pub fn use_post_manager(cx: &ScopeState) -> &UseAtomRef<PostManager> {
    use_atom_ref(cx, crate::app::POST_MANAGER)
}

#[derive(Default)]
pub struct PostManager {
    // NOTE An `IndexMap` is a map that preserves insertion order
    pub posts: IndexMap<PostId, PublicPost>,
}

impl PostManager {
    pub fn update<F>(&mut self, id: PostId, mut update_fn: F) -> bool
    where
        F: FnMut(&mut PublicPost),
    {
        if let Some(post) = self.posts.get_mut(&id) {
            update_fn(post);
            true
        } else {
            false
        }
    }

    pub fn populate<T>(&mut self, posts: T)
    where
        T: Iterator<Item = PublicPost>,
    {
        self.posts.clear();
        for post in posts {
            self.posts.insert(post.id, post);
        }
    }

    pub fn clear(&mut self) {
        self.posts.clear();
    }

    pub fn get(&self, post_id: &PostId) -> Option<&PublicPost> {
        self.posts.get(post_id)
    }

    pub fn remove(&mut self, post_id: &PostId) {
        self.posts.remove(post_id);
    }
}

pub fn PublicPostEntry(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);
    let router = use_router(cx);

    let this_post = {
        let post = post_manager.read().get(&post_id).unwrap().clone();
        use_state(cx, || post)
    };

    cx.render(rsx! {
        div {
            class: "grid grid-cols-[50px_1fr] gap-2 mb-4",
            div { /*profile image */}
            div {
                class: "flex flex-col gap-3",
                // header
                // reply to
                // content
                // action bar
                hr {}
            }
        }
    })
}
