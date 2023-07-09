use crate::authentication::handlers::get_claims::get_claims;
use crate::authentication::contract::errors::JwtAuthenticationError;

pub async fn get_authenticated_user_id(token: &str) -> Result<String, JwtAuthenticationError> {
    get_claims(token).await.map(|val| val.sub)
}