#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::prelude::*;

pub const BUTTON_SELECTED: &str = "border-slate-600";

// NOTE structs can have default generic types, the same will not be the case for functions
#[derive(Props)]
pub struct ButtonProps<'a, F = fn()>
where
    F: Fn(),
{
    #[props(default = false)]
    selected: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(default = BtnTypes::Button)]
    r#type: BtnTypes,
    #[props(default = None)]
    handle_onclick: Option<F>,
    class: Option<&'a str>,
    children: Element<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BtnTypes {
    Submit,
    Button,
}

impl BtnTypes {
    pub fn into_type(&self) -> &'static str {
        match self {
            BtnTypes::Submit => "submit",
            BtnTypes::Button => "button",
        }
    }
}

pub fn Button<'a, F>(cx: Scope<'a, ButtonProps<'a, F>>) -> Element<'a>
where
    F: Fn(),
{
    let disabled = cx.props.disabled;
    let btn_style = maybe_class!("btn-disabled", disabled);
    let class = &cx.props.class.unwrap_or_default();

    cx.render(rsx! {
        button {
            class: "btn {btn_style} {class}",
            r#type: cx.props.r#type.into_type(),
            disabled: disabled,
            onclick: move |_| {
                if let Some(callback) = &cx.props.handle_onclick {
                    callback();
                }
            },
            &cx.props.children
        }
    })
}
