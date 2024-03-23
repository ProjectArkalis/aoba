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

    async fn check_arkalis_auth(&mut self, token: String) -> Result<bool, AobaError> {
        let mut request = Request::new(GetUserInfoRequest {});
        request.metadata_mut().append(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
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
            })?;
        
        Ok(!response.into_inner().id.is_empty())
    }
}
