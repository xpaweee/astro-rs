use std::fs;
use std::io::Write;
use std::path::PathBuf;
use aes_gcm::{Aes256Gcm, aead::{Aead, generic_array::GenericArray}, KeyInit};
use aes_gcm::aes::Aes256;
use aes_gcm::aes::cipher::BlockEncrypt;
use serde_json::{json, Map, Value};
use serde_json::Value::String;


//TODO: ERROR HANDLING

pub trait TokenManager
{
    fn save_token(key: &str, value: &str) -> Result<(), ()>;
    fn get_token(key: &str) -> Result<String, ()>;

    fn get_config_path() -> PathBuf;
}

pub trait SecurityProfile
{
    fn encrypt(value: &str) -> Result<String, ()>;
    fn decrypt(value: &str) -> Result<String, ()>;

}

pub struct Aes256GcmProfile
{

}


impl SecurityProfile for Aes256GcmProfile
{
    fn encrypt(value: &str) -> Vec<u8> {
        const ENCRYPTION_KEY: &str = "your_very_secret_and_long_key_that_should_be_32_bytes!";
        const NONCE: u32 = 12;

        let key = GenericArray::from_slice(ENCRYPTION_KEY.as_bytes());
        let aes = Aes256Gcm::new(key);
        let result = aes.encrypt(GenericArray::from_slice(NONCE.to_string().as_ref()), value.as_bytes().as_ref()).expect("Encryption failure!");

        return result;
    }
    fn decrypt(value: &str) -> String {
        const ENCRYPTION_KEY: &str = "your_very_secret_and_long_key_that_should_be_32_bytes!";
        const NONCE: u32 = 12;

        let key = GenericArray::from_slice(ENCRYPTION_KEY.as_bytes());
        let aes = Aes256Gcm::new(key);
        let result = aes.decrypt(GenericArray::from_slice(NONCE.to_string().as_ref()), value.as_ref()).expect("Decryption failure");

        return String::from_utf8(result).expect("Decryption conversion error");
    }
}

impl TokenManager for Aes256GcmProfile
{

    fn save_token(key: &str, value: &str) -> Result<(), ()> {

        let config_path = Self::get_config_path();

        let mut file_data = if config_path.exists() {
            serde_json::from_str(&fs::read_to_string(&config_path)?).unwrap_or_else(|_| Map::new())
        }
        else {
            Map::new()
        };


        let encrypted_value = Self::encrypt(value);

        if file_data.contains_key(key)
        {
            println!("The proveded key already exists");
            Ok(())
        }

        file_data.insert(key.to_string(), Value::from(encrypted_value));


        let mut file = fs::File::create(config_path)?;
        file.write_all(json!(file_data).to_string().as_bytes())?;

        Ok(())

    }

    fn get_token(key: &str) -> Result<String, ()> {
        let config_path = Self::get_config_path();

        if !config_path.exists()
        {
            println!("Astro configuration dosen't exist");
            return Ok(())
        }
        let file = fs::read_to_string(&config_path).map_err(|_| ())?;
        const data: Map<String, Value> = serde_json::from_str(&file).unwrap_or_else(|_| Map::new());

        //TODO:
        Ok(data)

        // match data.get(key) {
        //  Some()
        // }, None =>
        // {
        //     println!("Value based on key dosent exist");
        //     Ok(())
        // }
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