use rocket::{fairing::{Fairing, Info, Kind}, http::Header, Request, Response};

mod users;

pub use crate::users::endpoints;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Access-Control-Allow-Origin",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8080"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![
            index,
            users::endpoints::get_users,
            users::endpoints::get_user,
            users::endpoints::user_options,
            users::endpoints::create_user,
            users::endpoints::update_user,
            users::endpoints::delete_user])
        .launch()
        .await?;

    Ok(())
}