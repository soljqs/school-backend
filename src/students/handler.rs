use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::swagger::responses::{ApiResponse, Page};
use crate::auth::extractor::AuthUser;

#[derive(Serialize, ToSchema)]
pub struct Student {
    pub id: i64,
    pub name: String,
    pub student_no: String,
    pub class: String,
}

#[utoipa::path(
    get,
    path = "/students",
    security(("bearerAuth" = [])),
    responses((status = 200, body = ApiResponse<Page<Student>>))
)]
#[get("/students")]
pub async fn list_students(
    _user: AuthUser,
    query: web::Query<(i64, i64)>,
) -> HttpResponse {
    let items = vec![
        Student {
            id: 1,
            name: "张三".into(),
            student_no: "2023001".into(),
            class: "计科一班".into(),
        }
    ];

    HttpResponse::Ok().json(ApiResponse::ok(Page {
        items,
        total: 1,
    }))
}

