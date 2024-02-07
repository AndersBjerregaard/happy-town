use rocket::{fairing::{Fairing, Info, Kind}, http::{Header, Status}, serde::{json::Json, Deserialize, Serialize}, Request, Response};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
// Users section

#[get("/user")]
async fn get_users() -> Json<String> {
    todo!()
}

#[get("/user/<id>")]
async fn get_user(id: String) -> Json<String> {
    todo!()
}

#[options("/user")]
fn user_options<'r>() -> Status {
    Status::Ok
}

#[post("/user", data = "<user>")]
async fn create_user(user: Json<User>) -> Json<String> {
    todo!()
}

#[put("/user", data = "<user>")]
async fn update_user(user: Json<User>) -> Json<String> {
    todo!()
}

#[delete("/user/<id>")]
async fn delete_user(id: String) -> Json<String> {
    todo!()
}

#[derive(Serialize,Deserialize)]
struct User {
    id: String,
    username: String,
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
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}