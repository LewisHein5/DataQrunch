use actix_web::Error;
use actix_web::dev::ServiceRequest;
use actix_web::http::header::HeaderValue;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};

use super::get_authenticated_user_id::get_authenticated_user_id;

pub async fn validate_request(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config: Config = req
        .app_data::<Config>()
        .cloned()
        .unwrap_or_default()
        .scope("scope"); //todo: scope

    let user_id = match get_authenticated_user_id(credentials.token()).await {
        Ok(v) => match HeaderValue::from_str(v.as_str()) {
            Ok(val) => val,
            Err(_) => return Err((AuthenticationError::from(config).into(), req))
        },
        Err(_) => return Err((AuthenticationError::from(config).into(), req)) //TODO: Do we need to parse the error here?
    };

    let mut new_req = ServiceRequest::from(req);
    new_req.headers_mut().insert("UserId".parse().unwrap(), user_id);

    Ok(new_req)
}