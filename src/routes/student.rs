use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::errors::AppError;
use crate::{
    state::AppState,
    models::{
        CreateStudent,
        UpdateStudent,
        Page,
        Student,
        ApiResponse,
    },
    services::student,
};
#[derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
pub async fn create_student_handler(
    state: web::Data<AppState>,
    payload: web::Json<CreateStudent>,
) -> Result<HttpResponse, AppError> {
    let s = student::create_student(&state.pool, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::ok(s)))
}
pub async fn list_students_handler(
    state: web::Data<AppState>,
    q: web::Query<PageQuery>,
) -> Result<HttpResponse, AppError> {
    let page = q.page.unwrap_or(1);
    let page_size = q.page_size.unwrap_or(10);

    if page <= 0 || page_size <= 0 {
        return Err(AppError::BadRequest(
            "page and page_size must be positive".into(),
        ));
    }

    let result = student::list_students(&state.pool, page, page_size).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::ok(result)))
}
pub async fn delete_student_handler(
    state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    student::delete_student(&state.pool, id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::ok(())))
}
pub async fn update_student_handler(
    state: web::Data<AppState>,
    path: web::Path<i64>,
    payload: web::Json<UpdateStudent>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let updated = student::update_student(
        &state.pool,
        id,
        payload.into_inner(),
    )
    .await?;

    if !updated {
        return Err(AppError::BadRequest("student not found".into()));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::ok(())))
}

