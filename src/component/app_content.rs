#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::rc::Rc;

use crate::{api::solana_service, component::transaction_form::transaction_form::TransactionForm};

pub fn AppContent<'a>(cx: Scope<'a>) -> Element<'a> {
    let is_loading: &'a UseState<bool> = use_state(cx, || false);

    let balance_future: &UseFuture<Result<String, reqwest::Error>> = use_future(cx, (), |_| {
        let loading = is_loading.clone();
        is_loading.set(true);

        async move {
            solana_service::get_balance()
                .await
                .and_then(move |response| {
                    loading.set(false);
                    Ok(response)
                })
        }
    });

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
                set_is_loading: is_loading.setter()
            },

            // Transaction form
            TransactionForm(cx, is_loading.setter()),
        }
    ))
}

#[component]
fn LoaderDisplay<'a>(cx: Scope, is_loading: &'a bool) -> Element {
    cx.render(match is_loading {
        true => rsx!(div {
            class: "spinner-border text-info"
        }),
        _ => rsx!(()),
    })
}

#[component]
fn DisplayBalance<'a>(
    cx: Scope<'a>,
    balance_future: &'a UseFuture<Result<String, reqwest::Error>>,
    set_is_loading: Rc<dyn Fn(bool)>,
) -> Element<'a> {
    cx.render(match balance_future.value() {
        Some(Ok(balance)) => {
            rsx!(
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
                    button {
                        class: "btn btn-info btn-lg ms-3",
                        onclick: |_| {
                            set_is_loading(true);

                            let cloned_set_is_loading = set_is_loading.clone();
                            async move {
                                match solana_service::get_5_sols().await {
                                    Ok(response) => {
                                        log::info!("{:?}", response);
                                        cloned_set_is_loading(false);
                                    },
                                    Err(e) => {
                                        log::info!("Failed to get solana service, {}", e);
                                        cloned_set_is_loading(false);
                                    }
                                }
                            }
                        },
                        "Get 5 Sols"
                    }
                }
            )
        }
        _ => rsx!(()),
    })
}
