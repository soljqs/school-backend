use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[post("/login")]
pub async fn login_handler(payload: web::Json<LoginReq>) -> HttpResponse {
    // ⚠️ demo：写死账号密码
    if payload.username == "admin" && payload.password == "123456" {
        return HttpResponse::Ok().json(json!({
            "token": "demo-token"
        }));
    }

    HttpResponse::Unauthorized().json(json!({
        "msg": "invalid credentials"
    }))
}

