#[allow(unused)]

use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::environment::Environment;
use crate::environment::Environment::{DEV, PROD, STAG};
use crate::errorr::ConfigError;

const CONFIG_FILENAME: &str = "conf/poem.toml";

pub type Result<T> = ::std::result::Result<T, ConfigError>;

#[derive(Debug)]
pub struct BasicConfig {
    pub environment: Environment,
    pub address: String,
    pub port: u16,
    pub workers: Option<u16>,
    pub db: Option<Database>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

#[derive(Debug, PartialEq)]
pub struct Database {
    pub(crate) adapter: String,
    pub(crate) db_name: String,
    pub(crate) pool: u32,
}

impl BasicConfig {
    pub fn new(env: Environment) -> Self {
        Self::default(env)
    }

    pub(crate) fn default(env: Environment) -> Self {
        let default_workers = (num_cpus::get() * 2) as u16;
        match env {
            DEV => {
                BasicConfig{
                    environment: env,
                    address: "localhost".to_string(),
                    port: 8000,
                    workers: Some(default_workers),
                    db: None,
                    config_file_path: None,
                    root_path: None,
                }
            },
            STAG=> {
                BasicConfig{
                    environment: env,
                    address: "0.0.0.0".to_string(),
                    port: 9000,
                    workers: Some(default_workers),
                    db: None,
                    config_file_path: None,
                    root_path: None,
                }
            },
            PROD=> {
                BasicConfig{
                    environment: env,
                    address: "0.0.0.0".to_string(),
                    port: 9000,
                    workers: Some(default_workers),
                    db: None,
                    config_file_path: None,
                    root_path: None,
                }
            },
        }
    }

    pub fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    fn default_from<P>(env: Environment, path: P) -> Result<Self>
    where P: AsRef<Path>
    {
        let mut config = BasicConfig::default(env);
        let config_file_path = path.as_ref().to_path_buf();
        if let Some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "config files must be rooted in a directory.";
            return Err(ConfigError::BadFilePath(config_file_path.clone(), msg));
        }
        config.config_file_path = Some(config_file_path);
        Ok(config)
    }
}

impl PartialEq for BasicConfig {
    fn eq(&self, other: &Self) -> bool {
           self.address == other.address
        && self.port    == other.port
        && self.workers == other.workers
    }
}

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub struct PoemConfig {
    pub active_env: Environment,
    config: HashMap<Environment, BasicConfig>,
}

impl PoemConfig {
    fn find() -> Result<PathBuf> {
        let current = env::current_dir().map_err(|_|ConfigError::NotFound)?;
        let mut pwd = current.as_path();
        loop {
            let manifest = pwd.join(CONFIG_FILENAME);
            if fs::metadata(&manifest).is_ok() {
                return Ok(manifest)
            }
            match pwd.parent() {
                Some(p) => pwd = p,
                None => break,
            }
        }
        Err(ConfigError::NotFound)
    }

    pub fn read_config() -> Result<PoemConfig> {
        let file = PoemConfig::find()?;
        let mut handle = File::open(&file).map_err(|_| ConfigError::IoError)?;
        let mut content = String::new();
        handle.read_to_string(&mut content).map_err(|_| ConfigError::IoError)?;
        PoemConfig::parse(content, &file)
    }

    pub fn active_default_from(filename: Option<&Path>) -> Result<PoemConfig> {
        let mut defaults = HashMap::new();
        if let Some(v) = filename {
            defaults.insert(DEV,  BasicConfig::default_from(DEV,  &v)?);
            defaults.insert(STAG, BasicConfig::default_from(STAG, &v)?);
            defaults.insert(PROD, BasicConfig::default_from(PROD, &v)?);
        } else {
            defaults.insert(DEV,  BasicConfig::default(DEV ));
            defaults.insert(STAG, BasicConfig::default(STAG));
            defaults.insert(PROD, BasicConfig::default(PROD));
        }

        let config = PoemConfig {
            active_env: Environment::active()?,
            config: defaults,
        };

        Ok(config)
    }

    pub fn active() -> Result<BasicConfig> {
        Ok(BasicConfig::new(Environment::active()?))
    }

    fn parse<P: AsRef<Path>>(content: String, file_name: P) -> Result<PoemConfig> {
        let path = file_name.as_ref().to_path_buf();
        let table = match content.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, found {}", value.type_str());
                return Err(ConfigError::ParseError(content, path, err, Some((1, 1))));
            }
            Err(e) => return Err(ConfigError::ParseError(content, path, e.to_string(), e.line_col()))
        };

        // Create a config with the defaults; set the env to the active one.
        let config = PoemConfig::active_default_from(Some(file_name.as_ref()))?;

        // Parse the values from the TOML file.
        for (entry, value) in table {
            // Each environment must be a table.
            let kv_pairs = match value.as_table() {
                Some(table) => table,
                None => return Err(ConfigError::BadType(
                    entry, "a table", value.type_str(), Some(path.clone())
                ))
            };
        }

        Ok(config)
    }
}