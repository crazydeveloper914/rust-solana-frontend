#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::api::solana_service;
use crate::util::basic_util;

pub fn AppContent(cx: Scope) -> Element {
    let is_loading = use_state(cx, || false);

    let on_set_is_loading = || {
        is_loading.modify(|value| !value);
    };

    let balance_future: &UseFuture<Result<String, reqwest::Error>> = use_future(
        cx,
        (),
        |_| async move { solana_service::get_balance().await },
    );

    cx.render(rsx!(
        div {
            class: "py-5 text-center text-white",
            // loader display
            LoaderDisplay{
                is_loading: is_loading.get(),
            },

            // Balance display
            DisplayBalance {
                balance_future: balance_future,
            },

            // Transaction form
            TransactionForm{},
        }
    ))
}

#[component]
fn LoaderDisplay<'a>(cx: Scope, is_loading: &'a bool) -> Element {
    cx.render(match is_loading {
        true => rsx!(div {
            class: "spinner-border text-info"
        },),
        _ => rsx!(()),
    })
}

#[component]
fn DisplayBalance<'a>(
    cx: Scope<'a>,
    balance_future: &'a UseFuture<Result<String, reqwest::Error>>,
) -> Element<'a> {
    cx.render(match balance_future.value() {
        Some(Ok(balance)) => rsx!(
            div {
                class: "d-flex align-items-center justify-content-center",
                div {
                    class: "display-2",
                    "Balance: {balance} SOL(s)",
                },
                button {
                    class: "btn btn-dark ms-3",
                    onclick: |_| { balance_future.restart(); },
                    i {
                        class: "bi bi-arrow-repeat fs-2 text-info"
                    }
                }
            }
        ),
        _ => rsx!(()),
    })
}

#[component]
fn TransactionForm(cx: Scope) -> Element {
    cx.render(rsx!(
        form {
            class: "container",
            onsubmit: move |event| {
                let prepared_values = basic_util::prepare_form_values(event.values.clone());

                async move {
                    if let Ok(response) = solana_service::transfer_sol(prepared_values).await {
                        log::info!("{}", response);
                    }else {
                        log::info!("Failed to transfer");
                    }
                }
            },

            // Amount to send (SOL) input
            InputGroup {
                field_type: "number".to_owned(),
                label: "Amount(in SOL) to send".to_owned(),
                id: "sol_to_send".to_owned(),
            }

            // send Sol to input
            InputGroup {
                field_type: "text".to_owned(),
                label: "Send SOL to".to_owned(),
                id: "to_pubkey".to_owned(),
            }

            // send button
            button {
                class: "btn btn-info btn-lg",
                r#type: "submit",
                "Send"
            }
        }
    ))
}

#[component]
fn InputGroup(cx: Scope, field_type: String, label: String, id: String) -> Element {
    cx.render(rsx!(
        div {
            class: "form-group my-5 text-start",
            label {
                class: "fs-3",
                r#for: &id[..],
                &label[..],
            }
            input {
                r#type: &field_type[..],
                id: &id[..],
                class: "form-control",
                name: &id[..],
                required: true,
            }
        }
    ))
}
