use super::{email, jwt};

pub fn build_login_email(address: String) -> email::Email {
    let jwt = jwt::sign_jwt(&address);

    email::Email {
        address,
        subject: "Your magic link arrived!".to_owned(),
        body: format!("http://localhost:3000/auth?token={}", jwt),
    }
}
