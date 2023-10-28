#![allow(non_snake_case)]
#![allow(unused)]

use futures_util::stream::StreamExt;

use dead_good_html_to_rsx_converter::convert;
use dioxus::html::textarea;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use log::{LevelFilter, warn};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();
    #[cfg(feature = "ssr")]
        let config = config.incremental(
        IncrementalRendererConfig::default().invalidate_after(std::time::Duration::from_secs(120)),
    );

    config.launch();
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {}
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    let parsed_text = use_state(cx, || "".to_string());

    let text_parser = use_coroutine(cx, |mut rx| {
        to_owned![parsed_text];
        async move {
            loop {
                match rx.next().await {
                    Some(text) => {
                        if let Ok(result) = convert(text) {
                            parsed_text.set(result);
                        }
                    }
                    None => {}
                }
            }
        }
    });

    cx.render(rsx! {
        nav {
            class: "shadow-sm p-8 w-full bg-neutral-50 dark:bg-neutral-900 dark:text-white",
            h1 { "Dead good HTML to RSX Converter" }
        }
        div {
            class: "flex flex-col md:flex-row w-full items-stretch justify-stretch h-full p-4 gap-4",
            textarea {
                class: "resize-none px-0 w-full h-[85vh] text-sm text-neutral-900 bg-white border-0 dark:bg-neutral-800 focus:ring-0 dark:text-white dark:placeholder-neutral-400",
                placeholder: "Paste HTML here",
                required: true,
                oninput: move |evt| text_parser.send(evt.value.clone()),
            }
            textarea {
                class: "resize-none px-0 w-full h-[85vh] text-sm text-neutral-900 bg-white border-0 dark:bg-neutral-800 focus:ring-0 dark:text-white dark:placeholder-neutral-400",
                placeholder: "RSX will appear here",
                required: true,
                value: "{parsed_text}",
            }
        }
    })
}
