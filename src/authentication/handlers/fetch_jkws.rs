use std::error::Error;
use alcoholic_jwt::JWKS;
use reqwest;
use crate::authentication::get_authority::get_authority;


pub(crate) async fn fetch_jwks() -> Result<JWKS, Box<dyn Error>> {
    let authority = get_authority();
    let uri = &format!("{}{}", authority.as_str(), ".well-known/jwks.json");
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;

    return Ok(val);
}