use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use dotenv::dotenv;
use std::env;

pub async fn auth<ReqBody>(req: Request<ReqBody>, next: Next<ReqBody>) -> impl IntoResponse {
    dotenv().ok();

    let token = req
        .headers()
        .get("token")
        .and_then(|header| header.to_str().ok());

    // 先把 token 存本地环境变量里
    let secret = env::var("TOKEN").expect("token is not found!");

    if let Some(tk) = token {
        if tk == &secret {
            return Ok(next.run(req).await);
        }
        return Err((StatusCode::UNAUTHORIZED, "token is incorrect".to_owned()));
    } else {
        return Err((StatusCode::UNAUTHORIZED, "token is not setting".to_owned()));
    }
}
