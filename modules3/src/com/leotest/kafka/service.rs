use crate::com::leotest::kafka::dao;
use crate::com::leotest::kafka::model::User;

pub fn get_user_by_id() -> User {
    dao::get_user_by_id()
}