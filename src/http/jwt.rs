use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub sub: Uuid,
    pub exp: i64,
    pub v: i32,
}

impl JwtToken {
    pub fn new(sub: Uuid, secret: &str, version: i32, expire: i64) -> Result<String> {
        let claims = Self {
            sub,
            exp: chrono::Utc::now().timestamp() + expire,
            v: version,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }
}

pub fn get_user_uuid(token: &str, secret: &str) -> Option<(Uuid, i32)> {
    let data = decode::<JwtToken>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;
    if data.claims.exp <= chrono::Utc::now().timestamp() { return None; }

    Some((data.claims.sub, data.claims.v))
}
