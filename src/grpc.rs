use crate::arkalis_service::arkalis_core_service_client::ArkalisCoreServiceClient;
use crate::config::AobaConfig;
use crate::errors::AobaError;
use std::str::FromStr;
use tonic::transport::{Channel, Uri};

pub type ArkalisGrpc = ArkalisCoreServiceClient<Channel>;

pub async fn get_client(config: &AobaConfig) -> Result<ArkalisGrpc, AobaError> {
    let channel = Channel::builder(
        Uri::from_str(config.arkalis_url.as_str()).map_err(|e| AobaError::Unknown(e.into()))?,
    )
    .connect()
    .await
    .map_err(|e| AobaError::ArkslisConnectionError(e.into()))?;
    let client = ArkalisCoreServiceClient::new(channel);
    Ok(client)
}
