#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::prelude::*;

pub const BUTTON_SELECTED: &str = "border-slate-600";

#[derive(Props, Debug)]
pub struct ButtonProps<'a> {
    #[props(default = false)]
    selected: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(default = BtnTypes::Button)]
    r#type: BtnTypes,
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

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let disabled = cx.props.disabled;
    let btn_style = maybe_class!("btn-disabled", disabled);

    cx.render(rsx! {
        button {
            class: "btn {btn_style}",
            r#type: cx.props.r#type.into_type(),
            disabled: disabled,
            &cx.props.children
        }
    })
}
