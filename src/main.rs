use rocket::{launch, routes};
use crate::user_controller::{delete, get_all, get_by_id, save, update};
mod user_controller;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![get_by_id,get_all,save,update,delete])
}