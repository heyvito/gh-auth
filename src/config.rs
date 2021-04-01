use std::env::var;
use std::fmt;
use std::env::VarError::NotPresent;

type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub struct ConfigError {
    pub message: String
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration error: {}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub github_key: String,
    pub github_secret: String,
    pub secret: String,
    pub allowed_users: Vec<String>,
}

fn get_env(name: &str) -> Result<String> {
    match var(name) {
        Ok(v) => {
            if v.trim().is_empty() {
                Err(ConfigError { message: format!("Environment variable must not be empty: {}", name) })
            } else {
                Ok(v)
            }
        }
        Err(err) => {
            if err == NotPresent {
                Err(ConfigError { message: format!("Environment variable not defined: {}", name) })
            } else {
                Err(ConfigError { message: format!("Environment variable must be composed by ASCII characters only: {}", name) })
            }
        }
    }
}

pub fn read_config<'a>() -> Result<Config> {
    let github_key = get_env("GITHUB_KEY")?;
    let github_secret = get_env("GITHUB_SECRET")?;
    let secret = get_env("SECRET_KEY")?;
    let allowed_users = get_env("ALLOWED_USERS")?;

    Ok(Config {
        github_key,
        github_secret,
        secret,
        allowed_users: allowed_users.split(",").map(|u| u.trim().to_string()).collect(),
    })
}
