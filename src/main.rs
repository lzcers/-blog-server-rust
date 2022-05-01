mod db;
mod layer;
mod services;

use axum::{
    handler::Handler,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use http::{request::Parts as RequestParts, HeaderValue};
use services::notes;
use std::{env, net::SocketAddr};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 4443));
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let notes_db = db::notes::NotesDB::new(&db_url)
        .await
        .expect("connect blog faild!");
    tracing::debug!("listening on {}", addr);

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &RequestParts| {
                origin.as_bytes().ends_with(b"ksana.net")
                    || origin.as_bytes().ends_with(b"localhost:3000")
            },
        ))
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .fallback(services::not_found.into_service())
        .route("/add_note", post(notes::add_note))
        .route("/update_note", post(notes::update_note))
        .route("/delete_note", post(notes::delete_note))
        // 上面路由都需要鉴权
        .route_layer(middleware::from_fn(layer::auth))
        .route("/auth_token", get(notes::auth_token))
        .route("/get_notes", post(notes::get_notes))
        .layer(Extension(notes_db))
        .layer(cors);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
