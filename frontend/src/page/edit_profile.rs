#![allow(non_snake_case)]

use crate::{
    components::{keyed_notification_box::KeyedNotifications, KeyedNotificationBox},
    prelude::*,
    util,
};
use dioxus::prelude::*;
use uchat_domain::UserFacingError;
use web_sys::HtmlInputElement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewImageData {
    DataUrl(String),
    Remote(String),
}

#[derive(Clone, Debug, Default)]
pub struct PageState {
    form_errors: KeyedNotifications,

    display_name: String,
    email: String,
    password: String,
    password_confirm: String,
    profile_image: Option<PreviewImageData>,
}

#[inline_props]
pub fn ImageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let toaster = use_toaster(cx);
    let handle_oninput = |_| {
        to_owned![page_state, toaster];
        async move {
            use gloo_file::{futures::read_as_data_url, File};
            use wasm_bindgen::JsCast;

            let el = util::document()
                .get_element_by_id("image-input")
                .unwrap()
                .unchecked_into::<HtmlInputElement>();
            let file: File = el.files().unwrap().get(0).unwrap().into();

            match read_as_data_url(&file).await {
                Ok(data) => page_state
                    .with_mut(|state| state.profile_image = Some(PreviewImageData::DataUrl(data))),
                Err(e) => {
                    toasty!(toaster => error: format!("Error loading file: {e}"));
                }
            }
        }
    };

    cx.render(rsx! {
        div {
            label { r#for: "image-input", "Upload Image" }
            input {
                class: "w-full",
                id: "image-input",
                r#type: "file",
                accept: "image/*",
                oninput: handle_oninput
            }
        }
    })
}

#[inline_props]
pub fn ImagePreview(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let image_data = page_state.with(|state| state.profile_image.clone());

    let img_el = |img_src| {
        rsx! { img { class: "profile-portrait-lg", src: "{img_src}" } }
    };

    let image_data = match image_data {
        Some(PreviewImageData::DataUrl(ref data)) => img_el(data),
        Some(PreviewImageData::Remote(ref url)) => img_el(url),
        None => rsx! {""},
    };

    cx.render(rsx! {image_data})
}

#[inline_props]
pub fn PasswordInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::user::Password;

    let check_password_mismatch = move || {
        let password_matches = page_state.with(|state| state.password == state.password_confirm);
        if password_matches {
            page_state.with_mut(|state| state.form_errors.remove("password-mismatch"));
        } else {
            page_state.with_mut(|state| {
                state
                    .form_errors
                    .set("password-mismatch", "Passwords must match")
            });
        }
    };

    cx.render(rsx! {
        fieldset { class: "fieldset",
            legend { "Set new password" }
            div { class: "flex w-full gap-2",
                div {
                    label { r#for: "password", "Password" }
                    input {
                        id: "password",
                        class: "input-field",
                        r#type: "password",
                        placeholder: "Password",
                        value: "{page_state.read().password}",
                        oninput: move |ev| {
                            match Password::new(&ev.value) {
                                Ok(_) => page_state.with_mut(|state| state.form_errors.remove("bad-password")),
                                Err(e) => {
                                    page_state
                                        .with_mut(|state| {
                                            state.form_errors.set("bad-password", e.formatted_error())
                                        })
                                }
                            }
                            page_state.with_mut(|state| state.password = ev.value.clone());
                            page_state.with_mut(|state| state.password_confirm = "".to_owned());
                            if page_state.with(|state| state.password.is_empty()) {
                                page_state.with_mut(|state| state.form_errors.remove("bad-password"));
                                page_state.with_mut(|state| state.form_errors.remove("password-mismatch"));
                            } else {
                                check_password_mismatch();
                            }
                        }
                    }
                }
                div {
                    label { r#for: "password-confirma", "Confirm Password" }
                    input {
                        id: "password-confirm",
                        class: "input-field",
                        r#type: "password",
                        placeholder: "Confirm Password",
                        value: "{page_state.read().password_confirm}",
                        oninput: move |ev| {
                            page_state.with_mut(|state| state.password_confirm = ev.value.clone());
                            check_password_mismatch();
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn EmailInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::user::Email;

    cx.render(rsx! {
        div {
            label { r#for: "email",
                div { class: "flex flex-row justify-between", span { "Email Address" } }
            }
            input {
                class: "input-field",
                id: "email",
                placeholder: "Email Address",
                value: "{page_state.read().email}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.email = ev.value.clone());
                    if ev.value.is_empty() {
                        page_state.with_mut(|state| state.form_errors.remove("bad-email"));
                        return;
                    }
                    match Email::new(&ev.value) {
                        Ok(_) => {
                            page_state.with_mut(|state| state.form_errors.remove("bad-email"));
                        }
                        Err(e) => {
                            page_state
                                .with_mut(|state| {
                                    state.form_errors.set("bad-email", e.formatted_error())
                                });
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn DisplayNameInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::user::DisplayName;

    let max_chars = DisplayName::MAX_CHARS;
    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().display_name.len() > max_chars
    );

    cx.render(rsx! {
        div {
            label { r#for: "display-name",
                div { class: "flex flex-row justify-between",
                    span { "Display Name" }
                    span { class: "text-right {wrong_len}", "{page_state.read().display_name.len()}/{max_chars}" }
                }
            }
            input {
                class: "input-field",
                id: "display-name",
                placeholder: "Display Name",
                value: "{page_state.read().display_name}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.display_name = ev.value.clone());
                    match DisplayName::new(&ev.value) {
                        Ok(_) => {
                            page_state.with_mut(|state| state.form_errors.remove("bad-display-name"));
                        }
                        Err(e) => {
                            page_state
                                .with_mut(|state| {
                                    state.form_errors.set("bad-display-name", e.formatted_error())
                                });
                        }
                    }
                }
            }
        }
    })
}

pub fn EditProfile(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let page_state = use_ref(cx, PageState::default);
    let router = use_router(cx);
    let toaster = use_toaster(cx);
    let local_profile = use_local_profile(cx);

    let _fetch_profile = {
        to_owned![api_client, toaster, page_state];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::user::endpoint::{GetMyProfile, GetMyProfileOk};
            let response = fetch_json!(<GetMyProfileOk>, api_client, GetMyProfile);
            match response {
                Ok(res) => page_state.with_mut(|state| {
                    state.display_name = res.display_name.unwrap_or_default();
                    state.email = res.email.unwrap_or_default();
                    state.profile_image = res
                        .profile_image
                        .map(|img| PreviewImageData::Remote(img.to_string()));
                }),
                Err(e) => toasty!(toaster => error: format!("Failed to retrieve posts: {e}")),
            }
        })
    };

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, page_state, router, toaster, local_profile],
        move |_| async move {
            use uchat_endpoint::user::endpoint::{UpdateProfile, UpdateProfileOk};
            use uchat_endpoint::Update;
            let request_data = {
                use uchat_domain::user::Password;
                UpdateProfile {
                    display_name: {
                        let name = page_state.with(|state| state.display_name.clone());
                        if name.is_empty() {
                            Update::SetNull
                        } else {
                            Update::Change(name)
                        }
                    },
                    email: {
                        let email = page_state.with(|state| state.email.clone());
                        if email.is_empty() {
                            Update::SetNull
                        } else {
                            Update::Change(email)
                        }
                    },
                    password: {
                        let password = page_state.with(|state| state.password.clone());
                        if password.is_empty() {
                            Update::NoChange
                        } else {
                            Update::Change(Password::new(password).unwrap())
                        }
                    },
                    profile_image: {
                        let profile_image = page_state.with(|state| state.profile_image.clone());
                        match profile_image {
                            Some(PreviewImageData::DataUrl(data)) => Update::Change(data),
                            Some(PreviewImageData::Remote(_)) => Update::NoChange,
                            None => Update::SetNull,
                        }
                    },
                }
            };

            let response = fetch_json!(<UpdateProfileOk>, api_client, request_data);
            match response {
                Ok(res) => {
                    toasty!(toaster => success: "Profile updated successfully!");
                    local_profile.write().image = res.profile_image;
                    router.navigate_to(page::HOME);
                }
                Err(e) => {
                    toasty!(toaster => error: format!("Failed to update profile: {e}"));
                }
            }
        }
    );
    let disable_submit = page_state.with(|state| state.form_errors.has_messages());

    cx.render(rsx! {
        AppBar { title: "Edit Profile",
            AppBarImgButton {
                handle_onclick: move |_| router.pop_route(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page"
            }
        }
        form { class: "flex flex-col w-full gap-3", onsubmit: form_onsubmit, prevent_default: "onsubmit",

            ImagePreview { page_state: page_state.clone() }
            ImageInput { page_state: page_state.clone() }
            DisplayNameInput { page_state: page_state.clone() }
            EmailInput { page_state: page_state.clone() }
            PasswordInput { page_state: page_state.clone() }

            KeyedNotificationBox { notifications: page_state.clone().read().form_errors.clone() }

            div { class: "flex justify-end gap-3",
                Button {
                    r#type: BtnTypes::Button,
                    handle_onclick: move || router.pop_route(),
                    "Cancel" 
                }
                // NOTE the ::<fn()> turbofish is needed to disambiguate the type of the Button
                Button::<fn()> {
                    r#type: BtnTypes::Submit,
                    disabled: disable_submit,
                    "Save"
                }
            }
        }
    })
}
