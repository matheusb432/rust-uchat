#![allow(non_snake_case)]

use dioxus::prelude::*;


pub const BUTTON_SELECTED: &str = "border-slate-600";

#[derive(Props, Debug)]
pub struct BarButtonProps<'a, F>
where
    F: Fn(Event<MouseData>),
{
    icon: &'static str,
    handle_onclick: F,
    children: Element<'a>,
}

pub fn BarButton<'a, F>(cx: Scope<'a, BarButtonProps<'a, F>>) -> Element
where
    F: Fn(Event<MouseData>),
{
    cx.render( rsx!{
                div { 
                    class: "grid grid-cols-[20px_1fr] gap-4 pl-4 justify-center items-center w-full h-12 border-y navbar-border-color", 
                    onclick: &cx.props.handle_onclick,
                    img { class: "invert", src: "{cx.props.icon}" }
                    &cx.props.children
                }
        })
}
