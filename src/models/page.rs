use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub data: Vec<T>,
}

