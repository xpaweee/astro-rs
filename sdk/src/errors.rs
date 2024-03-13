use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstroError {

#[error("Error")]
Error = 1,

#[error("The provided key already exists")]
ProvidedKeyAlreadyExists = 2,

#[error("Configuration already exists")]
ConfigurationDoesNotExist = 3,

#[error("An error occurred during encryption")]
EncryptionError = 4,

#[error("An error occurred during decryption")]
DecryptionError = 5,

#[error("Invalid data type")]
InvalidDataType = 6,

}