use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use http::header::{ACCEPT, ORIGIN};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::app::AppState;

/**
* This is basically a hello world route
*  that will be served at /hello
* Attach more routes and / or routers here
*  to expand the API
* Implicitly this is the v0 api of your project :)
*/

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}

async fn goodbye() -> impl IntoResponse {
    (StatusCode::OK, "Goodbye, World!")
}

pub fn router(state: AppState) -> Router<AppState> {
    // TODO: toughen up cors
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_headers(vec![ACCEPT, ORIGIN])
        .allow_origin(Any)
        .allow_credentials(false);

    Router::new()
        .route("/hello", get(hello))
        .route("/goodbye", get(goodbye))
        .with_state(state)
        .layer(cors_layer)
}
