#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::prelude::*;

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
}

// NOTE Moving state initialization to a separate struct
impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
        }
    }
}

// NOTE inline_props allows for a more succinct syntax for passing props in simple elements
#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div { class: "flex flex-col",
            label { r#for: "username", "Username" }
            input {
                id: "username",
                name: "username",
                class: "input-field",
                placeholder: "User Name",
                value: "{state.current()}",
                // NOTE Emitting an event to the parent component
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

// TODO refactor to reusable component
#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            label { r#for: "password", "Password" }
            input {
                id: "password",
                name: "password",
                r#type: "password",
                class: "input-field",
                placeholder: "Password",
                value: "{state.current()}",
                // NOTE Emitting an event to the parent component
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    let page_state = PageState::new(cx);
    // NOTE use_state only works with owned values, so use_ref is necessary to use borrowed values in state
    // ? It works by using a RefCell to store the value
    let page_state = use_ref(cx, || page_state);

    // NOTE sync_handler copies a pointer to the page state, making it available to the event handler efficiently
    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        // NOTE with_mut references the inner state value so that it's possible to mutate it
        page_state.with_mut(|state| state.username.set(ev.value.clone()));
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        page_state.with_mut(|state| state.password.set(ev.value.clone()));
    });

    cx.render(rsx! {
        form { class: "flex flex-col gap-5", prevent_default: "onsubmit", onsubmit: move |_| {},

            UsernameInput {
                // NOTE .with() passes an immutable reference of the state
                state: page_state.with(|state| state.username.clone()),
                // ? With the oninput event handler, this effectively creates 2-way databinding on the input
                oninput: username_oninput
            }
            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput
            }
            button {
                class: "btn",
                // ? since `type` is a reserved keyword, `r#` is necessary to set it
                r#type: "submit",
                "Signup"
            }
        }
    })
}
