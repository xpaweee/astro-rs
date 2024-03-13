
use std::string::String;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;
use aes_gcm::{Aes256Gcm, aead::{Aead, generic_array::GenericArray}, KeyInit};
use aes_gcm::aes::Aes256;
use aes_gcm::aes::cipher::BlockEncrypt;
use aes_gcm::aes::cipher::typenum::U12;
use base64::{decode, encode};
use serde_json::{json, Map, Value};
use serde_json::Value::{String as JsonString};
use crate::errors::AstroError;

pub struct TokenManager;

impl TokenManager
{
    pub fn save_token(&self, key: &str, value: &str) -> Result<(), AstroError> {

        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent()
        {
            fs::create_dir_all(parent).map_err(|_| AstroError::Error)?;
        }

        let mut file_data: Map<String, Value> = match fs::read_to_string(&config_path)
        {
            Ok(content) => serde_json::from_str(&content).map_err(|_| AstroError::Error)?,
            Err(_) => Map::new()
        };

        if file_data.contains_key(key)
        {
            return Err(AstroError::ProvidedKeyAlreadyExists);
        }

        let encrypted_key = Aes256GcmProfile::encrypt(value)?;

        file_data.insert(key.to_string(), Value::from(encrypted_key));
        let mut file = File::create(&config_path).map_err(|_| AstroError::Error)?;
        let file_data_json = serde_json::to_string(&file_data).map_err(|_| AstroError::Error);
        file.write_all(file_data_json.unwrap().as_bytes()).map_err(|_| AstroError::Error)?;

        Ok(())
    }

    pub fn get_token(&self, key: &str) -> Result<String, AstroError> {
        let config_path = Self::get_config_path();

        if !config_path.exists()
        {
           return Err(AstroError::ConfigurationDoesNotExist);
        }

        let file = fs::read_to_string(&config_path).map_err(|_| AstroError::Error)?;
        let file_data: Map<String, Value> = serde_json::from_str(&file).unwrap_or_else(|_| Map::new());
        let value = file_data.get(key);

        return if let Some(value) = value {
            let decrypted_value = Aes256GcmProfile::decrypt(value.as_str().ok_or(AstroError::InvalidDataType)?);
            Ok(decrypted_value?)
        } else {
            Err(AstroError::Error)
        }
    }

    fn get_config_path() -> PathBuf {
        let config_path: PathBuf = if cfg!(target_os = "windows") {
            PathBuf::from(std::env::var("USERPROFILE").unwrap()).join(".astro\\config")
        } else {
            PathBuf::from(std::env::var("HOME").unwrap()).join(".astro/config")
        };

        return config_path;
    }
}

pub trait SecurityProfile
{
    const ENCRYPTION_KEY: &'static str;
    fn encrypt(value: &str) -> Result<String, AstroError>;
    fn decrypt(value: &str) -> Result<String, AstroError>;
}

pub struct Aes256GcmProfile;

impl SecurityProfile for Aes256GcmProfile
{
    const ENCRYPTION_KEY: &'static str = "0123456789abcdef0123456789abcdef";

    fn encrypt(value: &str) -> Result<String, AstroError> {

        let nonce_bytes: [u8; 12] = [0; 12];
        let nonce = GenericArray::<u8, U12>::from_slice(&nonce_bytes);

        let key = GenericArray::from_slice(Self::ENCRYPTION_KEY.as_bytes());
        let aes = Aes256Gcm::new(key);
        let result = aes.encrypt(GenericArray::from_slice(nonce), value.as_bytes().as_ref());


        return result.map(|value| encode(value)).map_err(|_| AstroError::EncryptionError);
    }
    fn decrypt(value: &str) -> Result<String, AstroError> {

        let decoded_value = decode(value).unwrap();

        let nonce_bytes: [u8; 12] = [0; 12];
        let nonce = GenericArray::<u8, U12>::from_slice(&nonce_bytes);

        let key = GenericArray::from_slice(Self::ENCRYPTION_KEY.as_bytes());
        let aes = Aes256Gcm::new(key);
        let result = aes.decrypt(GenericArray::from_slice(nonce), decoded_value.as_ref()).map_err(|e| eprintln!("{}", e));

        return result.map(|result| String::from_utf8(result).unwrap()).map_err(|_| AstroError::DecryptionError);
    }
}