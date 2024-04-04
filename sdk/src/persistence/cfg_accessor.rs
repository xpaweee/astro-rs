use crate::errors::AstroError;
use serde::Deserialize;
use std::{fmt, fs};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub enum ProviderType {
    Gitlab,
    Github,
    AzurePipelines,
}

impl fmt::Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProviderType::Gitlab => write!(f, "Gitlab"),
            ProviderType::Github => write!(f, "Github"),
            ProviderType::AzurePipelines => write!(f, "AzurePipelines"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub token: String,
    pub provider: ProviderType,
}
pub struct ConfigAccessor {}

impl ConfigAccessor {
    pub fn new() -> Self {
        Self {}
    }

    fn get_config_path() -> PathBuf {
        const FILE_NAME: &'static str = "config2";

        let config_path: PathBuf = if cfg!(target_os = "windows") {
            PathBuf::from(std::env::var("USERPROFILE").unwrap())
                .join(format!(".astro\\{}", FILE_NAME))
        } else {
            PathBuf::from(std::env::var("HOME").unwrap()).join(format!(".astro/{}", FILE_NAME))
        };

        return config_path;
    }

    pub fn get_configuration_from_file(&self) -> Result<Vec<Config>, AstroError> {
        let path = Self::get_config_path();

        if !path.exists() {
            return Err(AstroError::ConfigurationDoesNotExist);
        }

        let file = fs::read_to_string(&path).map_err(|_| AstroError::Error)?;

        let configs: Vec<Config> = serde_json::from_str(&file).map_err(|e| AstroError::Error)?;

        Ok(configs)
    }
}
