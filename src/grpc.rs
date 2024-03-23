use crate::arkalis_service::arkalis_core_service_client::ArkalisCoreServiceClient;
use crate::errors::AobaError;
use tonic::transport::Channel;

pub type ArkalisGrpc = ArkalisCoreServiceClient<Channel>;

pub async fn get_client() -> Result<ArkalisGrpc, AobaError> {
    let channel = Channel::from_static("http://localhost:8000")
        .connect()
        .await
        .map_err(|e| AobaError::ArkslisConnectionError(e.into()))?;
    let client = ArkalisCoreServiceClient::new(channel);
    Ok(client)
}
