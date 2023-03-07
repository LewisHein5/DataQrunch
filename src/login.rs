use serde::{Serialize, Deserialize};
use actix_web::{web, Responder, Result};
use crate::session_key::SessionKey;
use crate::user_session_data::UserSessionData;
use crate::user_session_data_cache::UserSessionDataCache;

#[derive(Deserialize)]
pub(crate) struct LoginDto {
    user_name: String,
    password: String,
}
#[derive(Serialize)]
struct LoginResponse {
    status: String,
    session_key: String
}

pub(crate) async fn login(login_data: web::Json<LoginDto>, user_hash: web::Data<UserSessionDataCache>) -> actix_web::Result<impl Responder> {
    //TODO: Check that user exists
    //Todo: Validate password
    //Todo: Get user ID
    let key = SessionKey::new();
    let user_id: u64 = 1; //TODO: Fix this

    let user_data = UserSessionData::new(login_data.user_name.clone(), user_id);

    user_hash.add_session_key(key.clone(), user_data);

    let response = LoginResponse {
        status: String::from("OK"),
        session_key: key.into()
    };

    return Ok(web::Json(response));
}