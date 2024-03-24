use std::io::Cursor;
use std::path::{Path, PathBuf};

use rocket::fs::TempFile;
use rocket::http::ContentType;
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
    ) -> Result<String, AobaError> {
        self.check_arkalis_auth(token).await?;

        let content_type = file
            .content_type()
            .ok_or(AobaError::InvalidFileType)?
            .clone();

        if !content_type.is_png() && !content_type.is_jpeg() {
            return Err(AobaError::InvalidFileType);
        }

        let image_name = get_file_name(&content_type)?;

        file.copy_to(Self::get_upload_folder().await?.join(&image_name))
            .await
            .map_err(|e| AobaError::Unknown(e.into()))?;
        Ok(image_name)
    }

    pub async fn upload_imagem_from_url(
        &self,
        url: &str,
        token: String,
    ) -> Result<String, AobaError> {
        self.check_arkalis_auth(token).await?;

        let response = reqwest::get(url)
            .await
            .map_err(|_| AobaError::InvalidFileType)?;

        if response.content_length() > Some(25 * 1024 * 1024) {
            return Err(AobaError::InvalidFileType);
        }

        let content_type = response
            .headers()
            .get("Content-Type")
            .ok_or(AobaError::InvalidFileType)?
            .to_str()
            .map_err(|_| AobaError::InvalidFileType)?;
        let content_type =
            ContentType::parse_flexible(content_type).ok_or(AobaError::InvalidFileType)?;
        if !content_type.is_png() && !content_type.is_jpeg() {
            return Err(AobaError::InvalidFileType);
        }

        let file_name = get_file_name(&content_type)?;

        let mut file = tokio::fs::File::create(Self::get_upload_folder().await?.join(&file_name))
            .await
            .map_err(|e| AobaError::Unknown(e.into()))?;

        let mut content = Cursor::new(
            response
                .bytes()
                .await
                .map_err(|e| AobaError::Unknown(e.into()))?,
        );
        tokio::io::copy(&mut content, &mut file)
            .await
            .map_err(|e| AobaError::Unknown(e.into()))?;

        Ok(file_name)
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

fn get_file_name(content_type: &ContentType) -> Result<String, AobaError> {
    let ext = content_type.extension().ok_or(AobaError::InvalidFileType)?;
    Ok(format!("{}.{}", cuid2::cuid(), ext))
}
