use std::env;
use std::fmt;

pub struct Config {
    pub verify_token: String,
    pub access_token: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

impl Config {
    pub fn new() -> Config {
        let verify_token = env::var("VERIFY_TOKEN").unwrap_or_else(|| "NO_TOKEN".to_string());
        let access_token = env::var("ACCESS_TOKEN").unwrap_or_else(|| "".to_string());

        Config {
            verify_token,
            access_token,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Config {{ verify_token: {}, access_token: *** }}",
            &self.verify_token
        )
    }
}
