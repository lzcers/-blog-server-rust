use crate::db::{BlogDB, Note};
use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddNote {
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNote {
    id: i64,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteNote {
    id: i64,
}

pub async fn add_note(
    Json(payload): Json<AddNote>,
    Extension(blog_db): Extension<BlogDB>,
) -> Result<(StatusCode, Json<Note>), String> {
    let note = blog_db
        .add_note(payload.content)
        .await
        .expect("blog connecting faild!");

    Ok((StatusCode::OK, Json(note)))
}

pub async fn delete_note(
    Json(payload): Json<DeleteNote>,
    Extension(blog_db): Extension<BlogDB>,
) -> Result<StatusCode, String> {
    blog_db
        .delete_note(payload.id)
        .await
        .expect("delete note faild!");
    Ok(StatusCode::OK)
}

pub async fn update_note(
    Json(payload): Json<UpdateNote>,
    Extension(blog_db): Extension<BlogDB>,
) -> Result<StatusCode, String> {
    blog_db
        .update_note(payload.id, payload.content)
        .await
        .expect("update note faild!");
    Ok(StatusCode::OK)
}

pub async fn get_all_note(
    Extension(blog_db): Extension<BlogDB>,
) -> Result<(StatusCode, Json<Vec<Note>>), String> {
    let notes = blog_db.get_all_notes().await.expect("get all note faild!");
    Ok((StatusCode::OK, Json(notes)))
}
