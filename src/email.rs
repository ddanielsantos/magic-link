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
