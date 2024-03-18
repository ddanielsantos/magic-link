use dotenvy::{self};

pub enum Environment {
    Development,
    Production,
}

pub fn get_environment() -> Environment {
    let env = dotenvy::var("ENVIRONMENT");

    match env {
        Ok(env) => match env.as_str() {
            "production" | "prod" | "p" => Environment::Production,
            &_ => Environment::Development,
        },
        Err(_) => Environment::Development,
    }
}
