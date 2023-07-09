use std::future::Future;
use std::ops::Deref;
use actix_web::dev::ServiceRequest;
use actix_web::{Error, web};
use actix_web::error::InternalError;
use actix_web::http::header::HeaderValue;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use reqwest::header::HeaderName;
use crate::authentication::contract::claims::Claims;
use crate::authentication::contract::errors::JwtAuthenticationError;
use crate::authentication::handlers::get_claims::get_claims;
use crate::authentication::handlers::get_user_id::get_authenticated_user_id;
use crate::authentication::handlers::validate_token::validate_token;
use crate::user_session_data_cache::UserSessionDataCache;

pub async fn validate_request(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config: Config = req
        .app_data::<Config>()
        .cloned()
        .unwrap_or_default()
        .scope("scope"); //todo: scope

    let user_id = match get_authenticated_user_id(credentials.token()).await {
        Ok(v) => match HeaderValue::from_str(v.as_str()) {
            Ok(val) => val,
            Err(e) => return Err((AuthenticationError::from(config).into(), req))
        },
        Err(e) => return Err((AuthenticationError::from(config).into(), req))
    };

    let mut new_req = ServiceRequest::from(req);
    new_req.headers_mut().insert("UserId".parse().unwrap(), user_id);

    Ok(new_req)
}