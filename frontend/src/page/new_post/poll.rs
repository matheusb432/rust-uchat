#![allow(non_snake_case)]

use std::collections::BTreeMap;

use crate::{fetch_json, page::new_post_app_bar::NewPostAppBar, prelude::*, ret_if, toasty};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_domain::{
    ids::PollChoiceId,
    post::{PollChoiceDescription, PollHeadline},
};
use uchat_endpoint::post::types::{NewPostOptions, PollChoice};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub headline: String,
    // NOTE BTreeMap will sort the values by keys, it also has a very similar API to HashMap
    // ? Sorting the values by keys is necessary to display the poll choices correctly in the UI
    pub poll_choices: BTreeMap<usize, String>,
    pub next_id: usize,
}

impl Default for PageState {
    fn default() -> Self {
        Self {
            headline: "".to_owned(),
            poll_choices: {
                // TODO refactor to use  macro
                let mut map = BTreeMap::new();
                map.insert(0, "".to_owned());
                map.insert(1, "".to_owned());
                map
            },
            next_id: 2,
        }
    }
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        ret_if!(PollHeadline::new(&self.headline).is_err(), false);
        ret_if!(self.poll_choices.len() < 2, false);
        ret_if!(
            self.poll_choices
                .values()
                .map(PollChoiceDescription::new)
                .collect::<Result<Vec<PollChoiceDescription>, _>>()
                .is_err(),
            false
        );

        true
    }

    pub fn push_choice<T: Into<String>>(&mut self, choice: T) {
        self.poll_choices.insert(self.next_id, choice.into());
        self.next_id += 1;
    }

    pub fn replace_choice<T: Into<String>>(&mut self, key: usize, choice: T) {
        self.poll_choices.insert(key, choice.into());
    }
}

#[inline_props]
// TODO refactor to text input
pub fn HeadlineInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::PollHeadline;

    let max_chars = PollHeadline::MAX_CHARS;
    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().headline.len() > max_chars || page_state.read().headline.is_empty()
    );

    cx.render(rsx! {
        div {
            label { r#for: "headline",
                div { class: "flex flex-row justify-between",
                    span { "Headline" }
                    span { class: "text-right {wrong_len}", "{page_state.read().headline.len()}/{max_chars}" }
                }
            }
            input {
                class: "input-field",
                id: "headline",
                value: "{page_state.read().headline}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.headline = ev.data.value.clone());
                }
            }
        }
    })
}

#[inline_props]
pub fn PollChoices(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let choices = page_state.read().poll_choices.iter()
    .map(|(&key, choice)| {
        let choice = choice.clone();
        let max_chars = PollChoiceDescription::MAX_CHARS;
        let wrong_len = maybe_class!(
            "err-text-color",
            PollChoiceDescription::new(&choice).is_err()
        );
        // TODO refactor to remove duplication
        rsx! {
            li { key: "{key}",
                div { class: "grid grid-cols-[1fr_3rem_3rem] w-full gap-2 items-center h-8",
                    input {
                        class: "input-field",
                        placeholder: "Choice Description",
                        oninput: move |ev| {
                            page_state.with_mut(|state| state.replace_choice(key, &ev.data.value));
                        },
                        value: "{choice}"
                    }
                    div { class: "text-right {wrong_len}", "{choice.len()}/{max_chars}" }
                    Button {
                        class: "p-0 h-full bg-red-700",
                        r#type: BtnTypes::Button,
                        handle_onclick: move || {
                            page_state.with_mut(|state| state.poll_choices.remove(&key));
                        },
                        "X" 
                    }
                }
            }
        }
    }).collect::<Vec<LazyNodes>>();

    cx.render(rsx! {
        div { class: "flex flex-col gap-2",
            "Poll Choices"
            ol { class: "list-decimal ml-4 flex flex-col gap-2", choices.into_iter() }
            div { class: "flex flex-row justify-end",

                Button {
                    class: "w-1/2",
                    r#type: BtnTypes::Button,
                    handle_onclick: move || {
                        page_state.with_mut(|state| state.push_choice(""));
                    },
                    "+"
                }
            }
        }
    })
}

pub fn NewPoll(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);
    let is_invalid = !page_state.read().can_submit();
    let toaster = use_toaster(cx);
    let router = use_router(cx);
    let api_client = ApiClient::global();

    let form_onsubmit = async_handler!(
        &cx,
        [toaster, api_client, page_state, router],
        move |_| async move {
            use uchat_domain::post::PollHeadline;
            use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
            use uchat_endpoint::post::types::Poll;

            let read_ps = page_state.read();

            let request = NewPost {
                content: Poll {
                    headline: {
                        let headline = &read_ps.headline;
                        PollHeadline::new(headline).unwrap()
                    },
                    choices: {
                        // NOTE not necessary to sort the choices by key since the BTreeMap already does that
                        read_ps
                            .poll_choices
                            .values()
                            .map(|choice| PollChoice {
                                id: PollChoiceId::new(),
                                num_votes: 0,
                                description: PollChoiceDescription::new(choice).unwrap(),
                            })
                            .collect::<Vec<PollChoice>>()
                    },
                    voted: None,
                }
                .into(),
                options: NewPostOptions::default(),
            };

            let response = fetch_json!(<NewPostOk>, api_client, request);
            match response {
                Ok(_) => {
                    router.replace_route(page::HOME, None, None);
                    toasty!(toaster => success: "new post created!", 3);
                }
                Err(e) => {
                    toasty!(toaster => error: format!("Post failed: {e}"));
                }
            }
        }
    );

    cx.render(rsx! {
        NewPostAppBar { title: "New Poll".to_owned(), active_page: super::NewPostPages::Poll }
        form { class: "flex flex-col gap-4", onsubmit: form_onsubmit, prevent_default: "onsubmit",
            HeadlineInput { page_state: page_state.clone() }
            PollChoices { page_state: page_state.clone() }
            Button::<fn()> {
                r#type: BtnTypes::Submit,
                disabled: is_invalid,
                "Post"
            }
        }
    })
}
