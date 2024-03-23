use crate::grpc;
use crate::grpc::ArkalisGrpc;
use rocket::{async_trait, Build, Rocket};
use std::sync::Arc;

mod image_service;

pub(crate) type MutexGrpc = Arc<tokio::sync::Mutex<ArkalisGrpc>>;

#[async_trait]
pub trait AddServices {
    async fn add_services(self) -> Self;
}

#[async_trait]
impl AddServices for Rocket<Build> {
    async fn add_services(self) -> Self {
        let grpc = Arc::new(tokio::sync::Mutex::new(
            grpc::get_client()
                .await
                .expect("Failed to connect to Arkalis"),
        ));
        self.manage(grpc)
    }
}
