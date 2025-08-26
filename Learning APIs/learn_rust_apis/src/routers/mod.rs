
use axum::{
    routing::{self, get},
    Router
};

static let app = Router::new()
    .route("/", get(root)
    .route("/user", user));

async fn root() {

}

async fn user() {

}

