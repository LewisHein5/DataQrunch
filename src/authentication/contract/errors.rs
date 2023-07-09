use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
//todo: Refactor this, putting it in the authentication module violates seperation of concerns


#[derive(Debug, Display)]
pub(crate) enum JwtAuthenticationError {
    #[display(fmt = "Internal Server Error")]
    MalformedJwtError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,

    #[display(fmt = "KidDecodeError")]
    KidDecodeError,

    #[display(fmt = "KidNotFoundInSetError")]
    KidNotFoundInSetError,

    #[display(fmt = "UnauthorizedJwtError")]
    UnauthorizedJwtError
}

impl ResponseError for JwtAuthenticationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            JwtAuthenticationError::MalformedJwtError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            JwtAuthenticationError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            JwtAuthenticationError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JKWS")
            },
            JwtAuthenticationError::KidDecodeError => HttpResponse::InternalServerError().json("Internal Server Error"),
            JwtAuthenticationError::KidNotFoundInSetError => HttpResponse::InternalServerError().json("Internal Server Error"),
            JwtAuthenticationError::UnauthorizedJwtError => HttpResponse::Unauthorized().into()
        }
    }
}