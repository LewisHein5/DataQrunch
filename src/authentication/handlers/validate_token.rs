use alcoholic_jwt::{token_kid, Validation, validate};
use crate::authentication::contract::errors::JwtAuthenticationError;
use crate::authentication::handlers::fetch_jkws::fetch_jwks;
use crate::authentication::get_authority::get_authority;
use crate::authentication::handlers::get_claims::get_claims;

pub async fn validate_token(token: &str) -> Result<bool, JwtAuthenticationError> {
    get_claims(token).await.map(|_| true)
}