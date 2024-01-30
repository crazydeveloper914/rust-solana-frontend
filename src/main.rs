#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container py-5",
            div {
                "Hello, world!",
            }
            button {
                class: "btn btn-primary",
                onclick: |_| on_click_button(),
                r#type: "button",
                "Click Me"
            }
        }
    })
}

async fn on_click_button() {
    let response = reqwest::get("http://127.0.0.1:3000/get")
        .await
        .unwrap()
        .text()
        .await;

    log::info!("{:?}", response);
}
