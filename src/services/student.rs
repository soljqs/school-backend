use sqlx::MySqlPool;

use crate::data::student_repo;
use crate::models::{Student, CreateStudent, UpdateStudent, Page};
use crate::errors::AppError;

/* ---------------- list ---------------- */

pub async fn list_students(
    pool: &MySqlPool,
    page: i64,
    page_size: i64,
) -> Result<Page<Student>, AppError> {
    let (data, total) =
        student_repo::fetch_page(pool, page, page_size).await?;

    Ok(Page {
        page,
        page_size,
        total,
        data,
    })
}

/* ---------------- create ---------------- */

pub async fn create_student(
    pool: &MySqlPool,
    student: CreateStudent,
) -> Result<Student, AppError> {
    let s = student_repo::insert_student(pool, &student).await?;
    Ok(s)
}

/* ---------------- delete ---------------- */

pub async fn delete_student(
    pool: &MySqlPool,
    id: i64,
) -> Result<(), AppError> {
    student_repo::delete(pool, id).await?;
    Ok(())
}

/* ---------------- update ---------------- */

pub async fn update_student(
    pool: &MySqlPool,
    id: i64,
    payload: UpdateStudent,
) -> Result<bool, AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE students
        SET name = ?, age = ?, grade = ?
        WHERE id = ?
        "#,
        payload.name,
        payload.age,
        payload.grade,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

