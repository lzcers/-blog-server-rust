use crate::db;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

pub async fn auth<ReqBody>(req: Request<ReqBody>, next: Next<ReqBody>) -> impl IntoResponse {
    let token = req
        .headers()
        .get("token")
        .and_then(|header| header.to_str().ok());

    if let (Some(db), Some(tk)) = (req.extensions().get::<db::notes::NotesDB>(), token) {
        let auth_result = db.auth_token(tk).await.expect("token check faild");

        if auth_result == true {
            return Ok(next.run(req).await);
        } else {
            return Err((StatusCode::UNAUTHORIZED, "token is incorrect".to_owned()));
        }
    } else {
        return Err((StatusCode::UNAUTHORIZED, "token is not setting".to_owned()));
    }
}
