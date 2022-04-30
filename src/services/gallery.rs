use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::db::gallery::{GalleryDB, GalleryItem};

#[derive(Serialize, Deserialize)]
struct AddParams {
    url: String,
    datetime: String,
    localtion: String,
    description: String,
}

pub async fn get_items(
    Json(payload): Json<AddParams>,
    Extension(blog_db): Extension<GalleryDB>,
) -> Result<(StatusCode, Json<Vec<GalleryItem>>), (StatusCode, String)> {
    //     .add_note(payload.content, payload.created_at, payload.updated_at)
    //     .await
    //     .expect("blog connecting faild!");
    // Ok((StatusCode::OK, Json(note)))
    todo!()
}
