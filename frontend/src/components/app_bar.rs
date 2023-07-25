#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::prelude::{use_local_profile, use_sidebar};

pub const BUTTON_SELECTED: &str = "border-slate-600";

#[derive(Props)]
pub struct AppBarImgButtonProps<'a, F>
where
    F: Fn(Event<MouseData>),
{
    append_class: Option<&'a str>,
    click_handler: Option<F>,
    disabled: Option<bool>,
    img: &'a str,
    label: &'a str,
    title: Option<&'a str>,
}

#[derive(Props)]
pub struct AppBarProps<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn AppBarImgButton<'a, F>(cx: Scope<'a, AppBarImgButtonProps<'a, F>>) -> Element
where
    F: Fn(Event<MouseData>),
{
    let append_class = cx.props.append_class.unwrap_or("");
    let disabled = cx.props.disabled.unwrap_or(false);

    cx.render(rsx! {
        button {
            class: "flex flex-col w-10 h-14 justify-end items-center border-slate-200 border-b-4 {append_class}",
            disabled: disabled,
            onclick: move |ev| {
                if disabled {
                    return;
                }
                if let Some(callback) = &cx.props.click_handler {
                    callback(ev);
                }
            },
            title: cx.props.title,
            img { class: "w-6 h-6", src: cx.props.img }
            span { class: "text-sm", cx.props.label }
        }
    })
}

pub fn AppBar<'a>(cx: Scope<'a, AppBarProps<'a>>) -> Element {
    let local_profile = use_local_profile(cx);
    let sidebar = use_sidebar(cx);

    let local_profile = local_profile.read();
    let profile_img_src = local_profile
        .image
        .as_ref()
        .map(|url| url.as_str())
        .unwrap_or_else(|| "");

    cx.render(rsx! {
        div { class: "max-w-[var(--content-max-width)] h-[var(--appbar-height)] fixed top-0 right-0 left-0 mx-auto z-50 bg-slate-200",
            div { class: "flex gap-8 items-center w-full pr-5 h-full",
                div { class: "cursor-pointer", onclick: move |_| sidebar.write().open(), img { class: "profile-portrait", src: "{profile_img_src}" } }
                div { class: "text-xl font-bold mr-auto", "{cx.props.title}" }
                &cx.props.children
            }
        }
    })
}
