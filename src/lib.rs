#![allow(non_snake_case)]
#![allow(unused)]

use futures_util::stream::StreamExt;
use std::panic;

use dioxus::html::textarea;
use dioxus::prelude::*;
use dioxus_autofmt::write_block_out;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use html_parser::Dom;
use log::{warn, LevelFilter};
use serde::{Deserialize, Serialize};

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
                    if text.is_empty() {
                        // Shortcut if the text is empty
                        parsed_text.set(text);
                        continue;
                    }

                    let Ok(dom) = Dom::parse(text.trim()) else {
                        continue;
                    };

                    let body = rsx_rosetta::rsx_from_html(&dom);

                    let result = panic::catch_unwind(|| {
                        // This code has a lot of unwinds in it, so we need to catch them
                        write_block_out(body).expect("failed to write block out")
                    });

                    if let Ok(result) = result {
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
