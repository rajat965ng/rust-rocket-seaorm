use rocket::{delete, get, post, put, State};
use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel};
use sea_orm::ActiveValue::Set;

use crate::entities::post::{Model, NewPost};
use crate::entities::prelude::Post;

mod entities;


#[get("/post")]
pub async fn get_all(connection: &State<DatabaseConnection>) -> Json<Vec<Model>> {
    let connection = connection as &DatabaseConnection;
    let result =  Post::find().all(connection).await.unwrap();
    Json::from(result)
}


#[post("/post",format="json",data="<payload>")]
pub async fn save(connection: &State<DatabaseConnection>,payload:Json<NewPost>) -> Value {
    let connection = connection as &DatabaseConnection;
    let record = payload.into_inner().into_entity();
    let result =  Post::insert(record).exec(connection).await.unwrap();
    json!({"id":result.last_insert_id})
}

#[put("/post/<id>",format="json",data="<payload>")]
pub async fn update(connection: &State<DatabaseConnection>, id:i32, payload: Json<NewPost>) -> Json<Model> {
    let connection = connection as &DatabaseConnection;
    let record = Post::find_by_id(id).one(connection).await.unwrap();
    let mut record = record.unwrap().into_active_model();
    let payload = payload.into_inner();
    record.text = Set(payload.text);
    record.title = Set(payload.title);
    Json::from(record.update(connection).await.unwrap())
}


#[delete("/post/<id>")]
pub async fn delete(connection: &State<DatabaseConnection>, id:i32) -> Status {
    let connection = connection as &DatabaseConnection;
    let status =  if Post::delete_by_id(id).exec(connection).await.unwrap().rows_affected > 0 {
        Status::NoContent
    } else {
        Status::NotFound
    };
    status
}