use std::env;
use std::str::FromStr;
use crate::environment::Environment::{DEV, PROD, STAG};
use crate::errorr::ConfigError;

pub const CONFIG_ENV: &str = "POEM_ENV";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Environment {
    DEV,
    STAG,
    PROD,
}

impl Environment {
    pub fn active() -> Result<Environment, ConfigError> {
        match env::var(CONFIG_ENV) {
            Ok(s) => s.parse().map_err(|_| ConfigError::BadEnv(s)),
            // for Debug mode
            #[cfg(debug_assertions)]
            _ => Ok(DEV),
            // for Release mode
            #[cfg(not(debug_assertions))]
            _ => Ok(PROD),
        }
    }
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let env = match s {
            "d" | "dev"  | "devel" | "development" => DEV,
            "s" | "stag" | "staging" => STAG,
            "p" | "prod" | "production" => PROD,
            _ => return Err(()),
        };
        Ok(env)
    }
}