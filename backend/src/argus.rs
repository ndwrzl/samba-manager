use std::env::var;
use std::path::PathBuf;
pub struct EnvLog {
    pub main: String,
    pub file: PathBuf,
    pub user: String,
    pub password: String,
    pub api_key: Option<String>,
}

pub fn init_env() -> EnvLog {
    let main = var("LOG_FILE").expect("variable must be set");
    let user = var("LOGIN_USER").expect("variable must be set");
    let password = var("LOGIN_PASSWORD").expect("variable must be set");
    let api_key = var("API_KEY").ok();

    if api_key.is_none() {
        eprintln!("Warning: no API_KEY was set, API will be disabled.")
    }

    let file = PathBuf::from(&main);

    EnvLog {
        file,
        main,
        user,
        password,
        api_key,
    }
}
