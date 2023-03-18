#[derive(Clone)]
pub(crate) struct UserSessionData {
    pub(crate) user_name: String,
    pub(crate) user_id: u64,
}

impl UserSessionData {
    pub(crate) fn new<T: Into<u64>>(user_name: String, id: T) -> UserSessionData {
        return UserSessionData {
            user_name,
            user_id: id.into(),
        };
    }
}
