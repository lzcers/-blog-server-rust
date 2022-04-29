use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
mod db;
mod layer;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 4443));
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let blog_db = db::BlogDB::new(&db_url).await.expect("connect blog faild!");
    tracing::debug!("listening on {}", addr);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/add_note", post(services::add_note))
        .route("/update_note", post(services::update_note))
        .route("/delete_note", post(services::delete_note))
        // 上面路由都需要鉴权
        .route_layer(middleware::from_fn(layer::auth))
        .route("/get_all_notes", get(services::get_all_note))
        .layer(Extension(blog_db))
        .layer(cors);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
