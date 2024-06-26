use crate::config::AobaConfig;
use crate::grpc;
use crate::grpc::ArkalisGrpc;
use crate::services::image_service::ImageService;
use rocket::{async_trait, Build, Rocket};
use std::sync::Arc;

pub mod image_service;

pub type MutexGrpc = Arc<tokio::sync::Mutex<ArkalisGrpc>>;

#[async_trait]
pub trait AddServices {
    async fn add_services(self) -> Self;
}

#[async_trait]
impl AddServices for Rocket<Build> {
    async fn add_services(self) -> Self {
        let config = AobaConfig::new();

        let grpc = Arc::new(tokio::sync::Mutex::new(
            grpc::get_client(&config)
                .await
                .expect("Failed to connect to Arkalis"),
        ));

        self.manage(ImageService::new(grpc))
    }
}
