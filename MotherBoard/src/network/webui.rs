#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::net::SocketAddr;
use axum::{self, extract::WebSocketUpgrade, Router, routing::get, response::Html};
use super::get as nw_get;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        const HOSTNAME: &str = "raspiprint.local";
    } else {
        const HOSTNAME: &str = "localhost";
    }
}
const PORT: u16 = 8080;
const WILDCARD_IP: &str = "0.0.0.0";


fn App(cx: Scope) -> Element {
    let gcode = use_state(cx, || String::new());
    let state = use_state(cx, || nw_get().get_printer_state_ref());
    
    cx.render(rsx! {
        PageRoot {
            TitledBlock { title: "Upload"
                input {
                    r#type: "file",
                    accept: ".gcode, .txt, .rs, .md",
                    onchange: move |evt| {
                        let gcode = gcode.clone();
                        async move {
                            if let Some(file_engine) = &evt.files {
                                let files = file_engine.files();

                                for file_name in &files {
                                    if let Some(file) = file_engine.read_file_to_string(file_name).await {
                                        gcode.set(file);
                                    }
                                }
                            }
                        }
                    },
                },
                div {
                    class: "btn_div",
                    button {
                        onclick: |_| {nw_get().load_printer_gcode(gcode.as_str())},
                        "Submit!"
                    },
                },
            },
            TitledBlock { title: "Monitoring",
                strong { "Status {state:?}" }, br {},
                strong { "Temperature" }, br {},
                strong { "Completion" }, br {},
            },
            TitledBlock { title: "Controls",
                button {"Start"}, br {},
                button {"Pause"}, br {},
                button {"Stop"}, br {},
            },
        }
    })
}

#[inline_props]
fn PageRoot<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx!{
        div {
            class: "page_root",
            children
        }
    })
}

#[inline_props]
fn TitledBlock<'a>(cx: Scope<'a>, title: &'a str, children: Element<'a>) -> Element<'a> {
    cx.render(rsx!{
        div {
            class: "titled_block",
            h1 { "{title}" }, br {},
            children
        }
    })
}

pub async fn launch() {
    let ADDR = SocketAddr::new("0.0.0.0".parse().unwrap(), PORT);

    let skeleton = include_str!("skeleton.html");

    let view = dioxus_liveview::LiveViewPool::new();
    
    let router = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move || async move {
                Html(skeleton.replace("{glue}", &dioxus_liveview::interpreter_glue(&format!("ws://{}:{}/ws", HOSTNAME, PORT))))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view.launch(dioxus_liveview::axum_socket(socket), App).await;
                })
            }),
        );

    println!("P2I-6 DIY 3D-Printer listening on http://{}:{}", HOSTNAME, PORT);

    axum::Server::bind(&ADDR)
        .serve(router.into_make_service())
        .await
        .unwrap();
}