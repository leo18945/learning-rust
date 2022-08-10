use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use self::ConfigError::*;

#[derive(Debug)]
pub enum ConfigError {
    /// config file not found
    NotFound,
    /// There was an I/O error while reading the config file.
    IoError,
    /// The config file path was invalid.
    BadFilePath(PathBuf, &'static str),
    /// 'POEM_ENV' was invalid.
    BadEnv(String),
    /// table [environment] as invalid.
    BadEntry(String, PathBuf),
    /// a config key was specified with a value of the wrong type.
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    ///  there was a TOML parsing error.
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match self {
            NotFound=> "config file not found",
            IoError=> "There was an I/O error while reading the config file.",
            BadFilePath(..) => "The config file path was invalid.",
            BadEnv(..) => "'POEM_ENV' was invalid.",
            BadEntry(..) => "table [environment] as invalid.",
            BadType(..) => "a config key was specified with a value of the wrong type.",
            ParseError(..) => "there was a TOML parsing error.",
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotFound => write!(f, "config file was not found"),
            IoError => write!(f, "I/O error while reading the config file"),
            BadFilePath(ref p, _) => write!(f, "{:?} is not a valid config path", p),
            BadEnv(ref e) => write!(f, "{:?} is not a valid `ROCKET_ENV` value", e),
            BadEntry(ref e, _) => {
                write!(f, "{:?} is not a valid `[environment]` entry", e)
            }
            BadType(ref n, e, a, _) => {
                write!(f, "type mismatch for '{}'. expected {}, found {}", n, e, a)
            }
            ParseError(..) => write!(f, "the config file contains invalid TOML"),
        }
    }
}