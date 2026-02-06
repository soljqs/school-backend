use actix_web::{post, get, web, HttpResponse};
use serde::Deserialize;
use utoipa::ToSchema;
use serde::Serialize;
use crate::swagger::responses::ApiResponse;
use super::jwt::sign;
use super::extractor::AuthUser;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResp {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginReq,
    responses((status = 200, body = ApiResponse<LoginResp>))
)]
#[post("/auth/login")]
pub async fn login(req: web::Json<LoginReq>) -> HttpResponse {
    // demo：直接放行
    let token = sign(1, &req.username);
    HttpResponse::Ok().json(ApiResponse::ok(LoginResp { token }))
}

#[utoipa::path(
    get,
    path = "/auth/me",
    security(("bearerAuth" = [])),
    responses((status = 200, body = ApiResponse<LoginReq>))
)]
#[get("/auth/me")]
pub async fn me(user: AuthUser) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::ok(user))
}

