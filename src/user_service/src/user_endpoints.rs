use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
struct User {
    id: String,
    username: String,
}

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
