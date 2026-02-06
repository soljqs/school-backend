use actix_web::{web, HttpResponse};
use crate::AppState;
use crate::api_get;
use crate::swagger::responses::{
    StringListResponse,
    StringResponse,
    CountResponse,
};

api_get!(
    path = "/admin/db/tables",
    tag = "Admin DB",
    resp = StringListResponse
    =>
    pub async fn list_tables(
        _state: web::Data<AppState>,
    ) -> HttpResponse {
        HttpResponse::Ok().json(StringListResponse {
            data: vec![],
        })
    }
);

api_get!(
    path = "/admin/db/table/{table}/schema",
    tag = "Admin DB",
    resp = StringResponse,
    params = (
        ("table" = String, Path, description = "Table name")
    )
    =>
    pub async fn table_schema(
        table: web::Path<String>,
        _state: web::Data<AppState>,
    ) -> HttpResponse {
        HttpResponse::Ok().json(StringResponse {
            data: table.into_inner(),
        })
    }
);

api_get!(
    path = "/admin/db/table/{table}/count",
    tag = "Admin DB",
    resp = CountResponse,
    params = (
        ("table" = String, Path, description = "Table name")
    )
    =>
    pub async fn table_count(
        table: web::Path<String>,
        _state: web::Data<AppState>,
    ) -> HttpResponse {
        let _ = table;
        HttpResponse::Ok().json(CountResponse { data: 0 })
    }
);

