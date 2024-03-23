use rocket::launch;

use crate::controllers::AddControllers;
use crate::services::AddServices;

mod controllers;
mod errors;
mod grpc;
mod guards;
mod services;

pub mod arkalis_service {
    tonic::include_proto!("arkalis");
}

#[launch]
async fn rocket() -> _ {
    rocket::build().add_services().await.add_controllers()
}
