use crate::data::StarnDB;
use crate::page::success;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use rocket_db_pools::mongodb::bson;
use rocket_db_pools::Connection;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct User {
    name: String,
    pswd: String,
    role: String,
}

#[post("/starn/user/create", data = "<user>")]
pub async fn create_user(user: Json<User>) -> Json<success::Success<User>> {
    return Json(success::create(user.into_inner() as User));
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Users {
    users: Vec<User>,
    has_more: bool,
}

#[get("/starn/users")]
pub async fn users(mut db: Connection<StarnDB>) -> Json<success::Success<Users>> {
    return Json(success::create(Users {
        users: vec![],
        has_more: false,
    }));
}
