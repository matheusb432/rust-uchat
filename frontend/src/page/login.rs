#![allow(non_snake_case)]

use dioxus::prelude::*;
use uchat_domain::{self, UserFacingError};

use crate::{
    components::{
        keyed_notification_box::{KeyedNotificationBox, KeyedNotifications},
        local_profile,
    },
    fetch_json,
    prelude::*,
    toasty,
    util::ApiClient,
};

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        let is_form_empty =
            self.username.current().is_empty() || self.password.current().is_empty();
        let has_errors = self.form_errors.has_messages();

        !is_form_empty && !has_errors
    }
}

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
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

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
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

pub fn Login(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);
    let toaster = use_toaster(cx);
    let router = use_router(cx);
    let local_profile = use_local_profile(cx);

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, page_state, router, toaster, local_profile],
        move |_| async move {
            // NOTE Using the `Login` here will shadow the `Login` from the upper scope
            use uchat_endpoint::user::endpoint::{Login, LoginOk};
            let request_data = {
                use uchat_domain::{Password, Username};
                Login {
                    username: Username::new(
                        page_state.with(|state| state.username.current().to_string()),
                    )
                    .unwrap(),
                    password: Password::new(
                        page_state.with(|state| state.password.current().to_string()),
                    )
                    .unwrap(),
                }
            };

            let response = fetch_json!(<LoginOk>, api_client, request_data);
            match response {
                Ok(res) => {
                    let LoginOk {
                        session_expires,
                        session_id,
                        session_signature,
                        ..
                    } = res;
                    crate::util::cookie::set_session(
                        session_signature,
                        session_id,
                        session_expires,
                    );
                    local_profile.write().image = res.profile_image;
                    local_profile.write().user_id = Some(res.user_id);
                    router.navigate_to(page::HOME);
                }
                Err(e) => {
                    toasty!(toaster => error: format!("Failed to login: {e}"));
                }
            }
        }
    );

    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = uchat_domain::Username::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-username", e.formatted_error()))
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-username"))
        }

        page_state.with_mut(|state| state.username.set(ev.value.clone()));
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = uchat_domain::Password::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-password", e.formatted_error()))
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-password"))
        }

        page_state.with_mut(|state| state.password.set(ev.value.clone()));
    });

    let submit_btn_style =
        maybe_class!("btn-disabled", !page_state.with(|state| state.can_submit()));

    cx.render(rsx! {
        form { class: "flex flex-col gap-5", prevent_default: "onsubmit", onsubmit: form_onsubmit,
            UsernameInput {
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput
            }
            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput
            }
            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.with(|state| state.form_errors.clone())
            }
            button { class: "btn {submit_btn_style}", r#type: "submit", disabled: !page_state.with(|state| state.can_submit()), "Login" }
        }
    })
}
