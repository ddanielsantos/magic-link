use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Serialize;
use sha2::Sha256;

const JWT_SECRET: &str = "my_awesome_secret";

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub fn sign_jwt(address: &str) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_ref()).unwrap();
    let now = chrono::Utc::now();
    let expiration = (now + chrono::Duration::minutes(15)).timestamp() as usize;
    let issued_at = now.timestamp() as usize;

    let claims = Claims {
        sub: address.to_string(),
        exp: expiration,
        iat: issued_at,
    };

    claims.sign_with_key(&key).unwrap()
}
