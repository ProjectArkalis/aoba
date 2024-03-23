use rocket::http::Status;
use rocket::Request;
use rocket::response::Responder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AobaError {
    #[error("You do not have permission to perform this action")]
    Unauthorized,
    #[error("A connection error occurred with Arkalis")]
    ArkslisConnectionError(anyhow::Error),
    #[error("Unknown error")]
    Unknown(anyhow::Error),
    #[error("Invalid file type")]
    InvalidFileType,
}

impl<'r> Responder<'r, 'static> for AobaError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let resp = match self {
            AobaError::Unauthorized => (Status::new(401), self.to_string()),
            AobaError::ArkslisConnectionError(e) => (Status::new(500), e.to_string()),
            AobaError::Unknown(e) => (Status::new(500), e.to_string()),
            AobaError::InvalidFileType => (Status::new(400), self.to_string()),
        };
        
        log::error!("{}", resp.1);
        Err(resp.0)
    }
}