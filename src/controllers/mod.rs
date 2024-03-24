use rocket::fs::{relative, FileServer};
use rocket::{routes, Build, Rocket};

mod images;
mod test;

pub trait AddControllers {
    fn add_controllers(self) -> Self;
}

impl AddControllers for Rocket<Build> {
    fn add_controllers(self) -> Self {
        self.mount("/tests", routes![test::test])
            .mount("/images", routes![images::upload_image, images::upload_url])
            .mount("/images", FileServer::from(relative!("./uploads")))
    }
}
