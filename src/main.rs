#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};


#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", FileServer::from(relative!("client\\public")).rank(1))
}
