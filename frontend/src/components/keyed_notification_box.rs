#![allow(non_snake_case)]

use std::collections::{hash_map::Values, HashMap};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ret_if;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct KeyedNotifications {
    pub inner: HashMap<String, String>,
}

impl KeyedNotifications {
    pub fn set<K, V>(&mut self, k: K, v: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(k.into(), v.into());
    }

    // NOTE AsRef enables passing anything that can be converted to a str reference
    pub fn remove<K: AsRef<str>>(&mut self, k: K) {
        self.inner.remove(k.as_ref());
    }

    pub fn messages(&self) -> Values<'_, String, String> {
        self.inner.values()
    }

    pub fn has_messages(&self) -> bool {
        !self.inner.is_empty()
    }
}

#[derive(PartialEq, Props)]
pub struct KeyedNotificationsProps<'a> {
    legend: Option<&'a str>,
    notifications: KeyedNotifications,
}

pub fn KeyedNotificationBox<'a>(cx: Scope<'a, KeyedNotificationsProps<'a>>) -> Element {
    let notifications = cx.props.notifications.messages().map(|msg| {
        rsx! { li { "{msg}" } }
    });
    let legend = cx.props.legend.unwrap_or("Errors");

    ret_if!(!cx.props.notifications.has_messages());

    cx.render(rsx! {
        fieldset { class: "fieldset border-red-300 rounded",
            legend { class: "bg-red-300 px-4", "{legend}" }
            ul { class: "list-disc ml-4",
                // NOTE passing an iterator as children of `ul` will consume it when needed
                notifications
            }
        }
    })
}
