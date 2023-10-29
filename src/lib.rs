#![allow(non_snake_case)]
#![allow(unused)]

use futures_util::stream::StreamExt;

use dioxus::html::textarea;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use log::{warn, LevelFilter};
use serde::{Deserialize, Serialize};
use html_parser::Dom;
use dioxus_autofmt::write_block_out;

#[derive(Clone, Routable, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppRoutes {
    #[route("/")]
    Home {},
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    let parsed_text = use_state(cx, String::new);

    let text_parser = use_coroutine(cx, |mut rx| {
        to_owned![parsed_text];
        async move {
            loop {
                if let Some(text) = rx.next().await {
                    let text: String = text;
                    let dom = match Dom::parse(text.trim()) {
                        Ok(dom) => {
                            dom
                        }
                        Err(_) => {
                           continue;
                        }
                    };

                    let body = rsx_rosetta::rsx_from_html(&dom);

                    if let Some(result) = write_block_out(body) {
                        parsed_text.set(result);
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        nav {
            class: "font-mono p-4 w-full bg-neutral-50 dark:bg-neutral-900 dark:text-white",
            h1 {
                class: "font-mono text-xl",
                "Dead good HTML to RSX converter"
            }
        }
        div {
            class: "font-mono flex flex-col flex-grow md:flex-row items-stretch justify-stretch p-4 gap-4",
            textarea {
                class: "resize-none p-4 w-full h-full text-sm text-neutral-900 bg-neutral-100 border-0 dark:bg-neutral-800 focus:ring-0 dark:text-white dark:placeholder-neutral-400",
                placeholder: "Paste HTML here",
                required: true,
                aria_label: "Paste HTML here",
                oninput: move |evt| text_parser.send(evt.value.clone()),
            }
            textarea {
                class: "font-mono resize-none p-4 w-full h-full text-sm text-neutral-900 bg-neutral-100 border-0 dark:bg-neutral-800 focus:ring-0 dark:text-white dark:placeholder-neutral-400",
                placeholder: "RSX will appear here",
                aria_label: "RSX will appear here",
                required: true,
                value: "{parsed_text}",
            }
        }
        div {
            class: "text-center font-mono p-4 w-full",
            "🦊"
        }
    })
}
