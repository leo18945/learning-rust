use crate::com::leotest::kafka::model::User;

pub fn get_user_by_id() -> User {
    User {
        id: 1,
        name: "leo".to_string(),
    }
}