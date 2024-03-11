use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstroError {

#[error("Error")]
Error = 1,

#[error("The provided key already exists")]
ProvidedKeyAlreadyExists = 2

}