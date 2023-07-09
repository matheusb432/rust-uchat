#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn NewPostPopup(cx: Scope, hide: UseState<bool>) -> Element {
    let hide_class = maybe_class!("hidden", *hide.get());

    // TODO refactor BUTTON_CLASS div to component
    const BUTTON_CLASS: &str = "grid grid-cols-[20px_1fr] gap-4 pl-4 justify-center items-center w-full h-12 border-y navbar-border-color";

    cx.render( rsx!{
        div { class: "flex flex-col absolute right-0 bottom-[var(--navbar-height)] w-28 items-center {hide_class} navbar-bg-color text-white text-sm",
            div { class: BUTTON_CLASS, onclick: move |_| todo!(),
                img { class: "invert", src: "/static/icons/icon-poll.svg" }
                "Poll"
            }
            div { class: BUTTON_CLASS, onclick: move |_| todo!(),
                img { class: "invert", src: "/static/icons/icon-image.svg" }
                "Image"
            }
            div { class: BUTTON_CLASS, onclick: move |_| todo!(),
                img { class: "invert", src: "/static/icons/icon-messages.svg" }
                "Chat"
            }
        }
    })
}

#[derive(Props)]
pub struct NavButtonProps<'a> {
    img: &'a str,
    label: &'a str,
    onclick: EventHandler<'a, MouseEvent>,
    // NOTE The rsx! macro will allow this Option<T> to be passed in as T or be omitted entirely
    highlight: Option<bool>,
    children: Element<'a>,
}

pub fn NavButton<'a>(cx: Scope<'a, NavButtonProps<'a>>) -> Element {
    let selected_bgcolor = maybe_class!("bg-slate-500", matches!(cx.props.highlight, Some(true)));

    cx.render(rsx! {
        button {
            class: "cursor-pointer flex flex-col items-center justify-center h-full {selected_bgcolor}",
            onclick: move |ev| cx.props.onclick.call(ev),
            img { class: "invert", src: cx.props.img, width: "25px", height: "25px" }
            div { class: "text-sm text-white", cx.props.label }
            &cx.props.children
        }
    })
}

pub fn Navbar(cx: Scope) -> Element {
    let hide_new_post_popup = use_state(cx, || true);

    cx.render(rsx! {
        nav { class: "max-w-[var(--content-max-width)] h-[var(--navbar-height)] fixed bottom-0 left-0 right-0 mx-auto border-t navbar-bg-color navbar-border-color",
            div { class: "grid grid-cols-3 justify-around w-full h-full items-center shadow-inner",
                NavButton { img: "/static/icons/icon-home.svg", label: "Home", onclick: |_| () }
                NavButton { img: "/static/icons/icon-trending.svg", label: "Trending", onclick: |_| () }
                NavButton {
                    img: "/static/icons/icon-new-post.svg",
                    label: "Post",
                    onclick: |_| {
                        let is_hidden = *hide_new_post_popup.get();
                        hide_new_post_popup.set(!is_hidden);
                    },
                    NewPostPopup { hide: hide_new_post_popup.clone() }
                }
            }
        }
    })
}
