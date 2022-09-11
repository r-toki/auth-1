use crate::lib::jwt::Claims;
use crate::lib::jwt_extractor::AccessTokenDecoded;

pub fn authenticate(access_token_decoded: AccessTokenDecoded) -> anyhow::Result<Claims> {
    match access_token_decoded.0 {
        Some(claims) => Ok(claims),
        None => Err(anyhow::anyhow!("")),
    }
}
