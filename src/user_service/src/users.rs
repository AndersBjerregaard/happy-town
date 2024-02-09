pub mod endpoints {
    use rocket::{http::Status, serde::json::Json};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize,Deserialize)]
    pub struct User {
        id: String,
        username: String,
    }

    #[get("/user")]
    pub fn get_users() -> Json<String> {
        todo!()
    }
    
    #[get("/user/<id>")]
    pub fn get_user(id: String) -> Json<String> {
        todo!()
    }
    
    #[options("/user")]
    pub fn user_options<'r>() -> Status {
        Status::Ok
    }
    
    #[post("/user", data = "<user>")]
    pub fn create_user(user: Json<User>) -> Json<String> {
        todo!()
    }
    
    #[put("/user", data = "<user>")]
    pub fn update_user(user: Json<User>) -> Json<String> {
        todo!()
    }
    
    #[delete("/user/<id>")]
    pub fn delete_user(id: String) -> Json<String> {
        todo!()
    }
}
