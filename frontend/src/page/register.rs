#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;
use uchat_domain::{self, UserFacingError};

use crate::{
    components::{
        keyed_notification_box::{KeyedNotificationBox, KeyedNotifications},
        Button,
    },
    fetch_json,
    prelude::*,
    util::ApiClient,
};

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications,
    server_errors: KeyedNotifications,
}

// NOTE Moving state initialization to a separate struct
impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
            server_errors: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        let is_form_empty =
            self.username.current().is_empty() || self.password.current().is_empty();
        let has_errors = self.form_errors.has_messages();

        !is_form_empty && !has_errors
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

pub fn LoginLink(cx: Scope) -> Element {
    cx.render(rsx! {
        Link { class: "link text-center", to: page::ACCOUNT_LOGIN, "Existing User Login" }
    })
}

pub fn Register(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let page_state = PageState::new(cx);
    // NOTE use_state only works with owned values, so use_ref is necessary to use borrowed values in state
    // ? It works by using a RefCell to store the value
    let page_state = use_ref(cx, || page_state);
    let router = use_router(cx);
    let local_profile = use_local_profile(cx);

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, page_state, router, local_profile],
        move |_| async move {
            use uchat_endpoint::user::endpoint::{CreateUser, CreateUserOk};
            let request_data = {
                use uchat_domain::{Password, Username};
                page_state.with(|s| CreateUser {
                    username: Username::new(s.username.current().to_string()).unwrap(),
                    password: Password::new(s.password.current().to_string()).unwrap(),
                })
            };

            let response = fetch_json!(<CreateUserOk>, api_client,  request_data);

            match response {
                Ok(res) => {
                    crate::util::cookie::set_session(
                        res.session_signature,
                        res.session_id,
                        res.session_expires,
                    );
                    local_profile.write().user_id = Some(res.user_id);
                    router.navigate_to(page::HOME);
                }
                Err(e) => {
                    page_state.with_mut(|state| state.server_errors.set("register", e.to_string()));
                }
            };
        }
    );

    // NOTE sync_handler! custom macro copies a `page_state` pointer, making it available to the event handler efficiently
    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        // NOTE with_mut references the inner state value so that it's possible to mutate it
        page_state.with_mut(|s| {
            if let Err(e) = uchat_domain::Username::new(&ev.value) {
                s.form_errors.set("username", e.formatted_error())
            } else {
                s.form_errors.remove("username")
            };
            s.username.set(ev.value.clone());
        });
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        page_state.with_mut(|s| {
            if let Err(e) = uchat_domain::Password::new(&ev.value) {
                s.form_errors.set("password", e.formatted_error())
            } else {
                s.form_errors.remove("password")
            };
            s.password.set(ev.value.clone());
        });
    });

    // NOTE .with() passes an immutable reference of the state
    // * state: page_state.with(|state| state.username.clone()),
    // ? With the oninput event handler, this effectively creates 2-way databinding on the input
    cx.render(rsx! {
        form { class: "flex flex-col gap-5", prevent_default: "onsubmit", onsubmit: form_onsubmit,
            KeyedNotificationBox {
                legend: "Registration Errors",
                notifications: page_state.with(|state| state.server_errors.clone())
            }
            UsernameInput {
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput
            }
            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput
            }
            LoginLink {}
            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.with(|state| state.form_errors.clone())
            }
            Button {
                r#type: BtnTypes::Submit,
                disabled: !page_state.with(|state| state.can_submit()),
                "Signup"
            }
        }
    })
}
