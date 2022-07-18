#[derive(Debug)]
pub struct User {
    pub id   : u32,
    pub name : String,
}

pub trait To_String {
    fn to_string(&self) -> String;
}