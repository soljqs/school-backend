use actix_web::{FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};
use futures_util::future::{ready, Ready};
use utoipa::ToSchema;
use super::jwt::verify;
use serde::Serialize;
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthUser {
    pub user_id: i64,
    pub username: String,
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let Some(header) = req.headers().get("Authorization") else {
            return ready(Err(ErrorUnauthorized("Missing token")));
        };

        let token = header
            .to_str()
            .ok()
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| ErrorUnauthorized("Invalid token"));

        match token.and_then(|t| verify(t).map_err(|_| ErrorUnauthorized("Invalid token"))) {
            Ok(claims) => ready(Ok(AuthUser {
                user_id: claims.sub,
                username: claims.username,
            })),
            Err(e) => ready(Err(e)),
        }
    }
}

