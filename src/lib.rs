mod jwt {
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use serde::Serialize;
    use sha2::Sha256;

    const JWT_SECRET: &'static str = "my_awesome_secret";

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
}

pub mod env {
    pub enum Environment {
        Development,
        Production
    }

    pub fn get_environment() -> Environment {
        Environment::Development
    }
}

pub mod ses {
    use super::env::{Environment, get_environment};
    use super::jwt::sign_jwt;

    pub async fn send_email(address: &str) {
        let jwt = sign_jwt(address);

        let magic_link = format!("http://localhost:3000/auth?token={}", jwt);

        match get_environment() {
            Environment::Development => {
                println!("Sending email to {} with magic link: {}", address, magic_link);
            }
            Environment::Production => {
                todo!()
            }
        }
    }
}
