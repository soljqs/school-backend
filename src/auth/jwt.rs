use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

const SECRET: &[u8] = b"campus-secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: usize,
}

pub fn sign(user_id: i64, username: &str) -> String {
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        username: username.into(),
        exp,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap()
}

pub fn verify(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

