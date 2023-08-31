use axum::{
    routing::{get, post},
    Router,
};
use twist_server::{
    address,
    file::{exists_route, upload_route},
};

#[tokio::main]
async fn main() {
    twist_server::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/exists", post(exists_route))
        .route("/api/upload", post(upload_route));

    // run it with hyper on localhost:3000
    axum::Server::bind(&address())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
