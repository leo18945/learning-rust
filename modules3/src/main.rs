pub mod com;

use crate::com::leotest::kafka::controller::get_user_by_id;

fn main() {
    println!("Hello, world!");
    let user = get_user_by_id();
    println!("user->{:#?}", user);
}
