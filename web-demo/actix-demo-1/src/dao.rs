use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::io;
use sqlx::Row;
use crate::model::Course;

// pub async fn test1() -> io::Result<()> {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL 没有在.env文件中配置");
//
//     println!("database_url={}", database_url);
//
//     let db_pool = MySqlPoolOptions::new()
//
//         .connect(&database_url)
//         .await
//         .unwrap();
//
//     let course_rows =
//         sqlx::query("select id, teacher_id, name from course where id = ?")
//             .bind(1)
//             .fetch_all(&db_pool)
//             .await
//             .unwrap();
//
//     let mut course_list = vec![];
//
//     for row in course_rows {
//         course_list.push(
//             Course {
//                 id: row.get("id"),
//                 teacher_id: row.get("teacher_id"),
//                 name: row.get("name"),
//                 // time: chrono::Utc::now().
//                 // time: Some(chrono::NaiveDateTime::from(row.get("time").unwrap())),
//             }
//         )
//     }
//     println!("Courses = {:?}", course_list);
//     Ok(())
// }
//
// pub async fn test2() -> io::Result<()> {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL 没有在.env文件中配置");
//
//     println!("database_url={}", database_url);
//
//     let db_pool = MySqlPoolOptions::new()
//         .connect(&database_url)
//         .await
//         .unwrap();
//
//     let courses = sqlx::query_as::<_, Course>("select id, name, teacher_id, name from course")
//         .fetch_all(&db_pool)
//         .await
//         .unwrap();
//
//     for course in courses.iter() {
//         println!("course={:?}", course);
//     }
//
//     Ok(())
// }