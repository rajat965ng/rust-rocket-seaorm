use rocket::{delete, get, post, put};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Serialize, Debug)]
#[derive(Deserialize)]
#[serde(crate="rocket::serde")]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

#[get("/user/<id>")]
pub fn get_by_id(id: u64) -> Json<User> {
    Json(User{id, name: "Redhat".to_string(), email: "abc@gmail.com".to_string()})
}
#[get("/user")]
pub fn get_all() -> Json<Vec<User>> {
    let mut v = vec![];
    v.push(User{id:100, name: "Redhat".to_string(), email: "abc@gmail.com".to_string()});
    Json(v)
}
#[post("/user",format="json",data = "<user>")]
pub fn save(user:Json<User>) -> Json<User> {
    user
}
#[put("/user/<id>",format="json",data = "<user>")]
pub fn update(id: u64, mut user:Json<User>) -> Json<User> {
    user.id = id;
    println!("update {:?}",user.0);
    user
}
#[delete("/user/<id>")]
pub fn delete(id: u64) -> Status {
    println!("delete id {}",id);
    Status::NoContent
}
