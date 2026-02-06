use actix_web::web;
use sqlx::{Column, Row};
use serde_json::Value;

use crate::errors::AppError;
use crate::state::AppState;
use sqlx::TypeInfo;
const ALLOWED_TABLES: &[&str] = &[
    "students",
    "users",
    "logs",
];

fn check_table(table: &str) -> Result<(), AppError> {
    if ALLOWED_TABLES.contains(&table) {
        Ok(())
    } else {
        Err(AppError::BadRequest("table not allowed".into()))
    }
}
#[utoipa::path(
    get,
    path = "/admin/db/tables",
    tag = "Admin DB"
)]
pub async fn list_tables(
    state: web::Data<AppState>,
) -> Result<Vec<String>, AppError> {
    let rows = sqlx::query("SHOW TABLES")
        .fetch_all(&state.pool)
        .await?;

    let mut tables = vec![];

    for row in rows {
        let raw: Vec<u8> = row.try_get(0)?;
        let name = String::from_utf8_lossy(&raw).to_string();

        if ALLOWED_TABLES.contains(&name.as_str()) {
            tables.push(name);
        }
    }

    Ok(tables)
}
pub async fn table_schema(
    state: web::Data<AppState>,
    table: web::Path<String>,
) -> Result<Vec<Value>, AppError> {
    let table = table.into_inner();
    check_table(&table)?;

    let sql = format!("DESCRIBE `{}`", table);
    let rows = sqlx::query(&sql)
        .fetch_all(&state.pool)
        .await?;

    let data = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "field": r.try_get::<String,_>("Field").unwrap(),
                "type": r.try_get::<String,_>("Type").unwrap(),
                "null": r.try_get::<String,_>("Null").unwrap(),
                "key": r.try_get::<String,_>("Key").unwrap(),
                "default": r.try_get::<Option<String>,_>("Default").unwrap(),
            })
        })
        .collect();

    Ok(data)
}
pub async fn table_count(
    state: web::Data<AppState>,
    table: web::Path<String>,
) -> Result<i64, AppError> {
    let table = table.into_inner();
    check_table(&table)?;

    let sql = format!("SELECT COUNT(*) as cnt FROM `{}`", table);
    let row = sqlx::query(&sql)
        .fetch_one(&state.pool)
        .await?;

    Ok(row.try_get("cnt")?)
}
#[derive(serde::Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
#[utoipa::path(
    get,
    path = "/admin/db/{table}",
    tag = "Admin DB"
)]
pub async fn table_page(
    state: web::Data<AppState>,
    table: web::Path<String>,
    q: web::Query<PageQuery>,
) -> Result<Vec<Value>, AppError> {
    let table = table.into_inner();
    check_table(&table)?;

    let page = q.page.unwrap_or(1).max(1);
    let page_size = q.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let sql = format!(
        "SELECT * FROM `{}` LIMIT ? OFFSET ?",
        table
    );

    let rows = sqlx::query(&sql)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.pool)
        .await?;

    let sensitive = ["password", "token", "secret"];

    let mut data = vec![];

    for row in rows {
        let mut obj = serde_json::Map::new();

        for col in row.columns() {
            let name = col.name().to_string();
            let type_name = col.type_info().name().to_lowercase();

            let val = if sensitive.iter().any(|k| name.contains(k)) {
                Value::String("[hidden]".into())
            } else if type_name.contains("binary")
                || type_name.contains("blob")
            {
                Value::String("[binary]".into())
            } else {
                row.try_get::<Value, _>(name.as_str())
                    .unwrap_or(Value::Null)
            };

            obj.insert(name, val);
        }

        data.push(Value::Object(obj));
    }

    Ok(data)
}

