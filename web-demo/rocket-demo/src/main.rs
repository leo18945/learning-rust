// curl 高级技巧
// https://gist.github.com/ungoldman/11282441
// https://everything.curl.dev/http/post/json
// https://reqbin.com/req/c-dwjszac0/curl-post-json-example

// fn main() {
//     println!("Hello, world!");
// }
use rocket::{get, post, launch, routes, catch, catchers};
use std::error::Error;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;

// #[macro_use] extern crate name;

/**
测试命令
curl -X GET http://localhost:8000/hello
返回结果
hello Rust
 */
#[get("/")]
async fn hello() -> String {
    "hello Rust".to_string()
}

/**
测试命令
curl -X GET http://localhost:8000/user/user
返回结果
{"age":20,"name":"leo","status":1}
 */
#[get("/user")]
async fn get_user() -> Value {
    json!({"name": "leo", "age": 20, "status": 1})
}

/**
测试命令
curl -X POST http://localhost:8000/user/1001
返回结果
id=1001
 */
#[post("/<id>")]
async fn add_user(id: i32) -> String {
    let msg = format!("id={}", id);
    println!("{}", msg);
    msg
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct User {
    id   : u32,
    name : String,
    age  : u32
}

/**
测试命令
curl -X GET http://localhost:8000/user/man
返回结果
{"id":1001,"name":"leo","age":20}
*/
#[get("/man")]
async fn get_user2() -> Option<Json<User>> {
    Some(Json(User{
        id   : 1001,
        name : "leo".to_string(),
        age  : 20,
    }))
}

/**
测试命令
curl -X POST http://localhost:8000/user/ex \
     -H "Content-Type: application/json" \
     -d '{"id": 10010, "name": "leo", "age": 32}'
返回结果
{"data":"[id=10010, name=leo, age=32]"}
*/
#[post("/ex", format = "json", data = "<user>")]
async fn get_ex(user: Json<User>) -> Value {
    let user = user.into_inner();
    let msg = format!("[id={}, name={}, age={}]", user.id, user.name, user.age);
    json!({"data": msg})
}

/**
测试命令
curl -X POST http://localhost:8000/user/ex3 \
     -H "Content-Type: application/json" \
     -d '{"id": 10010, "name": "leo", "age": 32}'
返回结果
{"id":10010,"name":"leo","age":28}
 */
#[post("/ex3", format = "json", data = "<user>")]
async fn get_ex3(mut user: Json<User>) -> Option<Json<User>> {
    let mut user = user.into_inner();
    user.age = 28;
    Some(Json(user))
}

// 启动方式一
// 启动命令: cargo run
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/hello", routes![hello])
// }

// 启动方式二
// 启动命令: 直接运行 main 函数
#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        // router-路由注册
        .mount("/hello", routes![hello])
        .mount("/user", routes![add_user, get_user, get_user2, get_ex, get_ex3])
        // 注册全局异常处理
        .register("/", catchers![handle_404])
        .launch()
        .await?;
    Ok(())
}

/**
测试命令
curl -X POST http://localhost:8000/user/1001/1234
返回结果
{"err_code":"404","err_msg":"resource not found","hint":"change your url"}
*/
#[catch(404)]
async fn handle_404() -> Value {
    json!({"err_code": "404", "err_msg": "resource not found", "hint": "change your url"})
}