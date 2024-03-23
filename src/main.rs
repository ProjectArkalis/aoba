mod services;
mod controllers;
mod guards;

use rocket::launch;
use crate::controllers::AddControllers;

pub mod arkalis_service {
    tonic::include_proto!("arkalis");
}


#[launch]
fn rocket() -> _ {
    rocket::build().add_controllers()
}