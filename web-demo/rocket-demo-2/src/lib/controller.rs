use std::collections::HashMap;
use rocket::{get, post, delete, put, launch, routes, catch, catchers, State};
use rocket::serde::json::{Json, Value};
use crate::dto::User;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::serde_json::json;
use rocket::tokio::sync::Mutex;

pub type UserStatus = Mutex<HashMap<u32, User>>;
pub type Messages<'r> = &'r State<UserStatus>;

/**
测试命令
curl -X POST http://localhost:8000/user/add \
     -H "Content-Type: application/json" \
     -d '{"id": 1001, "name": "leo", "age": 32}'
返回结果
add user success
*/
#[post("/add", format="json", data="<user>")]
pub async fn add_user(user: Json<User>, messages: Messages<'_>) -> String {
    let u2 = user.into_inner();
    let mut map = messages.lock().await;
    if map.contains_key(&u2.id) {
        "user already exists".to_string()
    } else {
        map.entry(u2.id).or_insert(u2);
        "add user success".to_string()
    }
}

/**
测试命令
curl -X PUT http://localhost:8000/user/update \
     -H "Content-Type: application/json" \
     -d '{"id": 1001, "name": "leo", "age": 25}'
返回结果
update user success
 */
#[put("/update", format="json", data="<user>")]
pub async fn update_user(user: Json<User>, messages: Messages<'_>) -> String {
    let u2 = user.into_inner();
    let mut map = messages.lock().await;
    if map.contains_key(&u2.id) {
        map.insert(u2.id, u2);
        "update user success".to_string()
    } else {
        "update user fail, user does not exists".to_string()
    }
}

/**
curl -X DELETE http://localhost:8000/user/delete/100
remove user fail, user dose not exists

curl -X DELETE http://localhost:8000/user/delete/1001
remove user success
 */
#[delete("/delete/<id>")]
pub async fn delete_user(id: u32, messages: Messages<'_>) -> String {
    let mut map = messages.lock().await;
    if map.contains_key(&id) {
        map.remove(&id);
        "remove user success".to_string()
    } else {
        "remove user fail, user dose not exists".to_string()
    }
}

/**
curl localhost:8000/user/get/0
{"id":0,"name":"","age":0}

curl localhost:8000/user/get/1001
{"id":1001,"name":"leo","age":32}
 */
#[get("/get/<id>")]
pub async fn get_user(id: u32, messages: Messages<'_>) -> Json<User> {
    let map = messages.lock().await;
    let none: Json<User> = Json(User {
        id : 0,
        name : "".to_string(),
        age  : 0,
    });

    if id == 0 {
        return none;
    }

    match map.get(&id) {
        None => none,
        Some(u) => Json(u.to_owned()),
    }
}
