use actix_web::{HttpRequest, HttpResponse};
use crate::log_error;
use uuid::Uuid;

pub(crate) fn get_user_bearer_token<'a>(req: HttpRequest) -> Result<&'a str, HttpResponse> {
    Ok(match req
        .headers()
        .get("Authorization") {
        None => {
            let error_json = log_error!("No authorization header past authentication middleware");
            return Err(HttpResponse::InternalServerError().body(error_json));
        },
        Some(val) => {
            match val.to_str() {
                Ok(v) => v,
                Err(e) => {
                    let error_json = log_error!("Authorization header exists but could not convert to string. Error message: {}", e.to_string());
                    return Err(HttpResponse::InternalServerError().body(error_json));
                }
            }
        }
    })
}
