#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::service::solana;

pub fn AppContent(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "py-5 text-center text-white",
            // Balance display
            DisplayBalance(cx),
            // Amount to send (SOL) input
            // send Sol to input
            // send button
        }
    ))
}

fn DisplayBalance(cx: Scope) -> Element {
    let balance_future = use_future(cx, (), |_| async move { solana::get_balance().await });

    cx.render(match balance_future.value() {
        Some(Ok(balance)) => rsx!(
            div {
                class: "display-2",
                "Balance: {balance} SOL(s)",
            }
        ),
        _ => rsx!(""),
    })
}
