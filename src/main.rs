#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use component::app_bar::AppBar;
use component::app_content::AppContent;
use dioxus::prelude::*;

mod util;
mod api;
mod model;
mod component;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let app_style = r#"
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        text-align: left;
        background-color: #282c34;
    "#;

    cx.render(rsx! {
        div {
            style: "{app_style}",
            AppBar{},
            AppContent{},
        }
    })
}

