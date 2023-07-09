use alcoholic_jwt::ValidJWT;
use crate::authentication::contract::errors::JwtAuthenticationError;
use crate::authentication::contract::errors::JwtAuthenticationError::MalformedJwtError;

pub(crate) struct Claims {
    sub: String,
    exp: u64
}

impl TryFrom<ValidJWT> for Claims {
    type Error = JwtAuthenticationError;

    fn try_from(value: ValidJWT) -> Result<Self, Self::Error> {
        return Ok(
            Claims {
                sub: value.claims.get("sub").ok_or(MalformedJwtError)?.to_string(),
                exp: value.claims.get("exp").ok_or(MalformedJwtError)?.as_u64().ok_or(MalformedJwtError)?
            }
        );
    }
}