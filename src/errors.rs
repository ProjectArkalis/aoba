use thiserror::Error;

#[derive(Error, Debug)]
pub enum AobaError {
    #[error("You do not have permission to perform this action")]
    Unauthorized,
    #[error("A connection error occurred with Arkalis")]
    ArkslisConnectionError(anyhow::Error)
}