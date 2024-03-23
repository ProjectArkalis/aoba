use std::path::{Path, PathBuf};

use rocket::fs::TempFile;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::{Code, Request};

use crate::arkalis_service::GetUserInfoRequest;
use crate::errors::AobaError;
use crate::services::MutexGrpc;

pub struct ImageService {
    grpc: MutexGrpc,
}

impl ImageService {
    pub fn new(grpc: MutexGrpc) -> Self {
        Self { grpc }
    }

    pub async fn upload_image<'r>(
        &self,
        file: &mut TempFile<'r>,
        token: String,
    ) -> Result<(), AobaError> {
        self.check_arkalis_auth(token).await?;

        let content_type = file
            .content_type()
            .ok_or(AobaError::InvalidFileType)?
            .clone();
        let ext = content_type.extension().ok_or(AobaError::InvalidFileType)?;

        let image_name = format!("{}.{}", cuid2::cuid(), ext);

        file.copy_to(Self::get_upload_folder().await?.join(image_name))
            .await
            .map_err(|e| AobaError::Unknown(e.into()))?;
        Ok(())
    }

    async fn get_upload_folder() -> Result<PathBuf, AobaError> {
        let path = Path::new("./uploads");

        if !path.exists() {
            tokio::fs::create_dir_all(path)
                .await
                .map_err(|e| AobaError::Unknown(e.into()))?;
        }

        tokio::fs::canonicalize(path)
            .await
            .map_err(|e| AobaError::Unknown(e.into()))
    }

    async fn check_arkalis_auth(&self, token: String) -> Result<(), AobaError> {
        let mut request = Request::new(GetUserInfoRequest {});
        request.metadata_mut().append(
            "authorization",
            format!("Bearer {token}")
                .parse()
                .map_err(|e: InvalidMetadataValue| AobaError::Unknown(e.into()))?,
        );

        let response = self
            .grpc
            .lock()
            .await
            .get_user_info(request)
            .await
            .map_err(|e| {
                if e.code() == Code::Unauthenticated {
                    AobaError::Unauthorized
                } else {
                    AobaError::ArkslisConnectionError(e.into())
                }
            })?
            .into_inner();

        if response.role != "admin" && response.role != "uploader" {
            return Err(AobaError::Unauthorized);
        }

        Ok(())
    }
}
