use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id   : u32,
    pub name : String,
    pub age  : u32,
}