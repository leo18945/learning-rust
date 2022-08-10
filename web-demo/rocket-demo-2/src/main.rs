use std::collections::HashMap;
use rocket::{get, post, launch, routes, catch, catchers, State};
use std::error::Error;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::serde_json::json;
use lib::controller::{add_user, update_user, delete_user, get_user};
use rocket::tokio::sync::Mutex;
use lib::dto::User;
use lib::controller::{UserStatus, Messages};

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        .manage(UserStatus::new(HashMap::new()))
        // router-路由注册
        .mount("/user", routes![add_user, update_user, delete_user, get_user])
        .launch()
        .await?;
    Ok(())
}