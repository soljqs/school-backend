use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Student {
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub grade: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateStudent {
    pub name: String,
    pub age: i32,
    pub grade: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudent {
    pub name: String,
    pub age: i32,
    pub grade: String,
}

