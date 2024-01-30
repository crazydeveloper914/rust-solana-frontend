#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn AppBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "text-white",
            "hello"
        }
    })
}
