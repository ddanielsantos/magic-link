pub enum Environment {
    Development,
    Production,
}

pub fn get_environment() -> Environment {
    Environment::Development
}
