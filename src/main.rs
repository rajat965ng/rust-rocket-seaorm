use rocket::{launch, routes};
use sea_orm::Database;

use crate::user_controller::{delete, get_by_id, save, update};

mod user_controller;
mod post_controller;
mod entities;

#[launch]
async fn rocket() -> _ {
    let connection = match Database::connect("postgresql://yourUser:changeit@localhost:5432/postgres".to_string()).await {
        Ok(connection) => connection,
        Err(e) => panic!("{:?}",e)
    };
    rocket::build()
        .manage(connection)
        .mount("/",routes![get_by_id,user_controller::get_all,save,update,delete,
            post_controller::get_all,post_controller::save,post_controller::update,post_controller::delete])
}