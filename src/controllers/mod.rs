use rocket::{Build, Rocket, routes};

mod test;

pub trait AddControllers {
    fn add_controllers(self) -> Self;
}

impl AddControllers for Rocket<Build> {
    fn add_controllers(self) -> Self {
        self.mount("/tests", routes![test::test])
    }
}