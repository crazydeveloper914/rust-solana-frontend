#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn AppBar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "navbar navbar-dark bg-dark",
            div {
                class: "container-fluid py-2",
                img {
                    width: "300px",
                    alt: "solana-hero",
                    src: "/solana_logo.png",
                }
            }
        }
    })
}
