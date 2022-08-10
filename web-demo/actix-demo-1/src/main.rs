pub mod controller;
pub mod service;
pub mod dao;
pub mod model;
pub mod dto;

use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
//     // dao::test1();
//     dao::test2();
    Ok(())
}