use crate::com::leotest::kafka::model::User;
use crate::com::leotest::kafka::service;

pub fn get_user_by_id() -> User {
    service::get_user_by_id()
}