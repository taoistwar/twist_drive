mod server_arguments;
mod server_constants;
use axum::{
    routing::{get, post},
    Router,
};
pub use server_arguments::*;
pub use server_constants::*;
use tower_http::services::ServeDir;

use std::path::Path;

use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

use crate::{
    address, {exists_route, upload_route},
};

pub async fn startup() {
    crate::init();
    // build our application with a single route

    let download_serve = ServeDir::new(DATA_DIR.get().unwrap());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/exists", post(exists_route))
        .route("/api/upload", post(upload_route))
        .nest_service("/api/download", download_serve);

    // run it with hyper on localhost:3000
    axum::Server::bind(&address())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
