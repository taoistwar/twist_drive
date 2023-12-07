mod server_arguments;
mod server_constants;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
pub use server_arguments::*;
pub use server_constants::*;
use tower_http::services::ServeDir;

use crate::{
    address, {exists_route, upload_route},
};

// TODO ENV,手动指定: host, port, data_dir
pub async fn startup() {
    crate::init();
    // build our application with a single route

    let download_serve = ServeDir::new(DATA_DIR.get().unwrap());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/exists", post(exists_route))
        .route("/api/upload", post(upload_route))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024 * 1024))
        .nest_service("/api/download", download_serve);

    // run it with hyper on localhost:3000
    axum::Server::bind(&address())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
