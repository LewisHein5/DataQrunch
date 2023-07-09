use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use crate::authentication::handlers::validate_token::validate_token;

pub async fn validate_request(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config: Config = req
        .app_data::<Config>()
        .cloned()
        .unwrap_or_default()
        .scope("scope"); //todo: scope

    match validate_token(credentials.token()).await {
        Ok(val) => {
            if val {
                Ok(req)
            }
            else {
                Err((AuthenticationError::from(config).into(), req))
            }
        },
        Err(_) => Err((AuthenticationError::from(config).into(), req))
    }
}