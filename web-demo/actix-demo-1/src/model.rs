use chrono::{NaiveDateTime};

#[derive(Debug, sqlx::FromRow)]
pub struct Course {
    pub id          : i32,
    pub teacher_id  : i32,
    pub name        : String,
    // pub time        : Option<NaiveDateTime>,
}