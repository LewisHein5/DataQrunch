use chashmap::{CHashMap, ReadGuard};
use crate::session_key::SessionKey;
use crate::user_session_data::UserSessionData;


#[derive(Clone)]
pub(crate) struct UserSessionDataCache {
    store: CHashMap<SessionKey, UserSessionData>
}


impl UserSessionDataCache {
    pub(crate) fn add_session_key(&self, key: SessionKey, user_data: UserSessionData) {
        let user_id =
        self.store.insert(key, user_data);
    }

    pub(crate) fn new() -> UserSessionDataCache {
        return UserSessionDataCache {
            store: CHashMap::<SessionKey, UserSessionData>::new()
        }
    }

    pub(crate) fn remove_session_key(&self, key: &SessionKey) {
        self.store.remove(key);
    }

    pub(crate) fn get_user(&self, key: &SessionKey) -> Option<ReadGuard<SessionKey, UserSessionData>> {
        return self.store.get(key);
    }

    pub(crate) fn session_key_is_valid(&self, key: &SessionKey) -> bool {
        return self.store.contains_key(key);
    }
}
