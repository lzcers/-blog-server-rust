use crate::db::notes::{Note, NotesDB, NotesPage};
use axum::{http::StatusCode, Extension, Json};
use http::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetNotes {
    page_size: i32,
    page_number: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AddNote {
    content: String,
    updated_at: Option<String>,
    created_at: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct AuthResult {
    result: bool,
}

pub async fn add_note(
    Json(payload): Json<AddNote>,
    Extension(blog_db): Extension<NotesDB>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    let note = blog_db
        .add_note(payload.content, payload.created_at, payload.updated_at)
        .await
        .expect("blog connecting faild!");
    Ok((StatusCode::OK, Json(note)))
}

pub async fn delete_note(
    Json(payload): Json<DeleteNote>,
    Extension(blog_db): Extension<NotesDB>,
) -> Result<StatusCode, String> {
    blog_db
        .delete_note(payload.id)
        .await
        .expect("delete note faild!");
    Ok(StatusCode::OK)
}

pub async fn update_note(
    Json(payload): Json<UpdateNote>,
    Extension(blog_db): Extension<NotesDB>,
) -> Result<StatusCode, String> {
    blog_db
        .update_note(payload.id, payload.content)
        .await
        .expect("update note faild!");
    Ok(StatusCode::OK)
}

pub async fn auth_token(
    headers: HeaderMap,
    Extension(blog_db): Extension<NotesDB>,
) -> Result<(StatusCode, Json<AuthResult>), String> {
    let token = headers
        .get("token")
        .and_then(|header| header.to_str().ok())
        .expect("token not found");

    let auth_result = blog_db.auth_token(token).await;
    if let Ok(result) = auth_result {
        Ok((StatusCode::OK, Json(AuthResult { result })))
    } else {
        Err("auth faild".to_owned())
    }
}

pub async fn get_notes(
    headers: HeaderMap,
    Json(payload): Json<GetNotes>,
    Extension(blog_db): Extension<NotesDB>,
) -> Result<(StatusCode, Json<NotesPage>), String> {
    let mut is_editor = false;
    if let Some(token) = headers.get("token").and_then(|header| header.to_str().ok()) {
        if true
            == blog_db
                .auth_token(token)
                .await
                .expect("get notes auth faild!")
        {
            is_editor = true;
        }
    }
    let notes = blog_db
        .get_notes(payload.page_number, payload.page_size, is_editor)
        .await
        .expect("get all note faild!");
    Ok((StatusCode::OK, Json(notes)))
}
