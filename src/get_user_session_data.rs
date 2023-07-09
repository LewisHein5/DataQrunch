use crate::session_key::SessionKey;
use crate::user_session_data::UserSessionData;
use crate::user_session_data_cache::UserSessionDataCache;

use actix_web::{web, HttpResponse};
use std::ops::Deref;

pub(crate) fn get_user_session_data(
    session_key: &String,
    user_data_cache: web::Data<UserSessionDataCache>,
) -> Result<UserSessionData, HttpResponse> {
    let access_denied_response = HttpResponse::Unauthorized().body("");
    let session_key = match SessionKey::try_from(session_key) {
        Ok(val) => val,
        Err(_) => return Err(access_denied_response),
    };

    if !user_data_cache.session_key_is_valid(&session_key) {
        return Err(access_denied_response);
    }

    let user_session_data = match user_data_cache.get_user(&session_key) {
        Some(val) => val.deref().clone(),
        None => return Err(access_denied_response),
    };

    return Ok(user_session_data);
}
