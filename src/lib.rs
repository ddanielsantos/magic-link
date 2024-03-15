mod jwt {
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
}

pub mod env {
    pub enum Environment {
        Development,
        Production,
    }

    pub fn get_environment() -> Environment {
        Environment::Development
    }
}

pub mod user_mailing {
    use super::{email, jwt};

    pub fn build_login_email(address: String) -> email::Email {
        let jwt = jwt::sign_jwt(&address);

        email::Email {
            address,
            subject: "Your magic link arrived!".to_owned(),
            body: format!("http://localhost:3000/auth?token={}", jwt),
        }
    }
}

pub mod email {
    use axum::http::StatusCode;

    use super::env::{get_environment, Environment};

    #[trait_variant::make(EmailSenderVariant: Send)]
    pub trait EmailSender {
        async fn send_email(email: &Email) -> Result<(), EmailSenderError>;
    }

    pub struct SESWrapper {}

    pub struct Email {
        pub address: String,
        pub subject: String,
        pub body: String,
    }

    #[derive(Debug)]
    pub enum EmailSenderError {
        Unknown,
    }

    impl axum::response::IntoResponse for EmailSenderError {
        fn into_response(self) -> axum::response::Response {
            match self {
                EmailSenderError::Unknown => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to send email, please try again.",
                )
                    .into_response(),
            }
        }
    }

    impl EmailSender for SESWrapper {
        async fn send_email(email: &Email) -> Result<(), EmailSenderError> {
            match get_environment() {
                Environment::Development => {
                    println!(
                        "Sending email to {} with subject {}, with the following body: \n{}",
                        email.address, email.subject, email.body
                    );

                    Ok(())
                }
                Environment::Production => Err(EmailSenderError::Unknown),
            }
        }
    }
}
