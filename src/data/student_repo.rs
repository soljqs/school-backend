use sqlx::MySqlPool;
use crate::models::{Student, CreateStudent};

pub async fn insert_student(
    pool: &MySqlPool,
    s: &CreateStudent,
) -> Result<Student, sqlx::Error> {
    let id = sqlx::query!(
        r#"INSERT INTO students (name, age, grade) VALUES (?, ?, ?)"#,
        s.name,
        s.age,
        s.grade
    )
    .execute(pool)
    .await?
    .last_insert_id() as i64;

    Ok(Student {
        id,
        name: s.name.clone(),
        age: s.age,
        grade: s.grade.clone(),
    })
}

pub async fn delete(pool: &MySqlPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM students WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn fetch_page(
    pool: &MySqlPool,
    page: i64,
    page_size: i64,
) -> Result<(Vec<Student>, i64), sqlx::Error> {
    let offset = (page - 1) * page_size;

    let total = sqlx::query!("SELECT COUNT(*) as count FROM students")
        .fetch_one(pool)
        .await?
        .count as i64;

    let list = sqlx::query_as!(
        Student,
        r#"
        SELECT id, name, age, grade
        FROM students
        ORDER BY id DESC
        LIMIT ? OFFSET ?
        "#,
        page_size,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok((list, total))
}

