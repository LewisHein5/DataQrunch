use alcoholic_jwt::{token_kid, validate, Validation};
use crate::authentication::contract::claims::Claims;
use crate::authentication::contract::errors::JwtAuthenticationError;
use crate::authentication::contract::errors::JwtAuthenticationError::{MalformedJwtError, JWKSFetchError, KidDecodeError, KidNotFoundInSetError, UnauthorizedJwtError};
use crate::authentication::get_authority::get_authority;
use crate::authentication::handlers::fetch_jkws::fetch_jwks;

pub (crate) async fn get_claims(token: &str) -> Result<Claims, JwtAuthenticationError>{
    let kid = match token_kid(&token) {
        Ok(val) => val.ok_or(KidDecodeError)?,
        Err(_) => return Err(JWKSFetchError)
    };

    let jwks = match fetch_jwks().await {
        Ok(val) => val,
        Err(E) => return Err(JWKSFetchError)
    };

    let validations = vec![Validation::Issuer(get_authority()), Validation::SubjectPresent];
    let jwk = jwks.find(&kid).ok_or(KidNotFoundInSetError)?;

    let res = validate(token, jwk, validations).map_err(|_| UnauthorizedJwtError)?;

    return Claims::try_from(res);
}