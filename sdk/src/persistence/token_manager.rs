
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;
use aes_gcm::{Aes256Gcm, aead::{Aead, generic_array::GenericArray}, KeyInit};
use aes_gcm::aes::Aes256;
use aes_gcm::aes::cipher::BlockEncrypt;
use aes_gcm::aes::cipher::typenum::U12;
use serde_json::{json, Map, Value};
use crate::errors::AstroError;

pub struct TokenManager;


impl TokenManager
{
    pub fn save_token(&self, key: &str, value: &str) -> Result<(), AstroError> {

        let config_path = Self::get_config_path();

        let mut file_data: Map<String, Value> = match fs::read_to_string(&config_path)
        {
            Ok(content) => serde_json::from_str(&content).map_err(|_| AstroError::Error)?,
            Err(_) => Map::new()
        };

        if file_data.contains_key(key)
        {
            return Err(AstroError::Error);
        }


        let encrypted_key = Aes256GcmProfile::encrypt(value);

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
            println!("Astro configuration dosen't exist");
            return Ok("s".parse().unwrap());
        }
        let file = fs::read_to_string(&config_path).map_err(|_| AstroError::Error)?;
        let data: Map<String, Value> = serde_json::from_str(&file).unwrap_or_else(|_| Map::new());

        // println!("{}", data);
        //TODO:
        // Ok(data)

        Ok("a".to_string())
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

pub trait SecurityProfile
{
    fn encrypt(value: &str) -> Vec<u8>;
    fn decrypt(value: &str) -> String;

}

pub struct Aes256GcmProfile
{

}


impl SecurityProfile for Aes256GcmProfile
{
    fn encrypt(value: &str) -> Vec<u8> {
        const ENCRYPTION_KEY: &str = "0123456789abcdef0123456789abcdef";

        let nonce_bytes: [u8; 12] = [0; 12];
        let nonce = GenericArray::<u8, U12>::from_slice(&nonce_bytes);

        let key = GenericArray::from_slice(ENCRYPTION_KEY.as_bytes());
        let aes = Aes256Gcm::new(key);
        let result = aes.encrypt(GenericArray::from_slice(nonce), value.as_bytes().as_ref()).expect("Encryption failure!");

        return result;
    }
    fn decrypt(value: &str) -> String {
        const ENCRYPTION_KEY: &str = "test";
        const NONCE: u32 = 12;

        let key = GenericArray::from_slice(ENCRYPTION_KEY.as_bytes());
        let aes = Aes256Gcm::new(key);
        let result = aes.decrypt(GenericArray::from_slice(NONCE.to_string().as_ref()), value.as_ref()).expect("Decryption failure");

        return String::from_utf8(result).expect("Decryption conversion error");
    }
}




//
// use std::fs;
// use std::fs::File;
// use std::io::{Error, Write};
// use std::path::PathBuf;
// use aes_gcm::{Aes256Gcm, aead::{Aead, generic_array::GenericArray}, KeyInit};
// use aes_gcm::aes::Aes256;
// use aes_gcm::aes::cipher::BlockEncrypt;
// use serde_json::{json, Map, Value};
// use crate::errors::AstroError;
//
//
//
// pub trait TokenManager
// {
//     fn save_token(key: &str, value: &str) -> Result<(), AstroError>;
//     fn get_token(key: &str) -> Result<String, AstroError>;
//
//     fn get_config_path() -> PathBuf;
// }
//
// pub trait SecurityProfile
// {
//     fn encrypt(value: &str) -> Vec<u8>;
//     fn decrypt(value: &str) -> String;
//
// }
//
// pub struct Aes256GcmProfile
// {
//
// }
//
//
// impl SecurityProfile for Aes256GcmProfile
// {
//     fn encrypt(value: &str) -> Vec<u8> {
//         const ENCRYPTION_KEY: &str = "your_very_secret_and_long_key_that_should_be_32_bytes!";
//         const NONCE: u32 = 12;
//
//         let key = GenericArray::from_slice(ENCRYPTION_KEY.as_bytes());
//         let aes = Aes256Gcm::new(key);
//         let result = aes.encrypt(GenericArray::from_slice(NONCE.to_string().as_ref()), value.as_bytes().as_ref()).expect("Encryption failure!");
//
//         return result;
//     }
//     fn decrypt(value: &str) -> String {
//         const ENCRYPTION_KEY: &str = "your_very_secret_and_long_key_that_should_be_32_bytes!";
//         const NONCE: u32 = 12;
//
//         let key = GenericArray::from_slice(ENCRYPTION_KEY.as_bytes());
//         let aes = Aes256Gcm::new(key);
//         let result = aes.decrypt(GenericArray::from_slice(NONCE.to_string().as_ref()), value.as_ref()).expect("Decryption failure");
//
//         return String::from_utf8(result).expect("Decryption conversion error");
//     }
// }
//
// impl TokenManager for Aes256GcmProfile
// {
//
//     fn save_token(key: &str, value: &str) -> Result<(), AstroError> {
//
//         let config_path = Self::get_config_path();
//
//        let mut file_data: Map<String, Value> = match fs::read_to_string(&config_path)
//        {
//            Ok(content) => serde_json::from_str(&content).map_err(|_| AstroError::Error)?,
//            Err(_) => Map::new()
//        };
//
//         if file_data.contains_key(key)
//         {
//             return Err(AstroError::Error);
//         }
//
//
//         let encrypted_key = Self::encrypt(value);
//
//         file_data.insert(key.to_string(), Value::from(encrypted_key));
//
//
//         let mut file = File::create(&config_path).map_err(|_| AstroError::Error)?;
//         let file_data_json = serde_json::to_string(&file_data).map_err(|_| AstroError::Error);
//
//         file.write_all(file_data_json.unwrap().as_bytes()).map_err(|_| AstroError::Error)?;
//
//         Ok(())
//     }
//
//
//
//     fn get_token(key: &str) -> Result<String, AstroError> {
//         let config_path = Self::get_config_path();
//
//         if !config_path.exists()
//         {
//             println!("Astro configuration dosen't exist");
//             return Ok("s".parse().unwrap());
//         }
//         let file = fs::read_to_string(&config_path).map_err(|_| AstroError::Error)?;
//         let data: Map<String, Value> = serde_json::from_str(&file).unwrap_or_else(|_| Map::new());
//
//         // println!("{}", data);
//         //TODO:
//         // Ok(data)
//
//         Ok("a".to_string())
//         // match data.get(key) {
//         //  Some()
//         // }, None =>
//         // {
//         //     println!("Value based on key dosent exist");
//         //     Ok(())
//         // }
//     }
//
//     fn get_config_path() -> PathBuf {
//         let config_path: PathBuf = if cfg!(target_os = "windows") {
//             PathBuf::from(std::env::var("USERPROFILE").unwrap()).join(".astro\\config")
//         } else {
//             PathBuf::from(std::env::var("HOME").unwrap()).join(".astro/config")
//         };
//
//         return config_path;
//     }
// }