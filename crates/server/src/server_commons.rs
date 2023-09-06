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

pub fn init_logs() {
    if MODE.get().unwrap().eq_ignore_ascii_case("dev") {
        let file = "log4rs-dev.yaml";
        if Path::new(file).exists() {
            let msg = format!("{} init failed", file);
            log4rs::init_file(file, Default::default()).expect(&msg);
            return;
        }
        init_defined_logs(LevelFilter::Debug);
        return;
    }
    let file = "log4rs-prod.yaml";
    if Path::new(file).exists() {
        let msg = format!("{} init failed", file);
        log4rs::init_file(file, Default::default()).expect(&msg);
        return;
    }
    let file = "log4rs-rel.yaml";
    if Path::new(file).exists() {
        let msg = format!("{} init failed", file);
        log4rs::init_file(file, Default::default()).expect(&msg);
        return;
    }
    init_defined_logs(LevelFilter::Info);
}

fn init_defined_logs(level: LevelFilter) {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();

    let config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    log4rs::init_config(config).expect("default log4rs init failed");
}
