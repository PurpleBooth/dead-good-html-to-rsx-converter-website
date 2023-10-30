#![allow(non_snake_case)]
#![allow(unused)]

use dead_good_html_to_rsx_converter_web::AppRoutes;
use dioxus::html::textarea;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use futures_util::stream::StreamExt;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

use dioxus_fullstack::router::FullstackRouterConfig;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to initialize logger");
    let config = LaunchBuilder::<FullstackRouterConfig<AppRoutes>>::router();
    config.launch();
}

#[cfg(not(target_arch = "wasm32"))]
use shuttle_axum::ShuttleAxum;

#[cfg(not(target_arch = "wasm32"))]
use shuttle_runtime::main;

#[allow(clippy::unused_async)]
#[cfg(not(target_arch = "wasm32"))]
#[main]
async fn loader() -> ShuttleAxum {
    use axum::routing::*;
    use axum::{Router, ServiceExt};
    use dioxus::prelude::*;
    use dioxus_fullstack::prelude::*;
    use shuttle_axum::AxumService;
    use shuttle_runtime::ResourceBuilder;
    use tower_http::trace::TraceLayer;
    use tower_http::{compression::CompressionLayer, BoxError};

    use shuttle_runtime::Context;
    let router = Router::new()
        .serve_dioxus_application(
            "",
            ServeConfigBuilder::new_with_router(FullstackRouterConfig::<AppRoutes>::default()),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());
    Ok(AxumService::from(router))
}
