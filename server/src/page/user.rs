use crate::data::StarnDB;
use crate::page::resp;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use rocket_db_pools::mongodb::bson::doc;
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
pub async fn create_user(user: Json<User>) -> Json<resp::Resp<User>> {
    return Json(resp::create(
        0,
        "success".to_string(),
        user.into_inner() as User,
    ));
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Users {
    name: String,
}

#[get("/starn/users")]
pub async fn users(db: Connection<StarnDB>) -> Json<resp::Resp<Users>> {
    let res = match db.list_database_names(doc! {}, None).await {
        Ok(r) => r,
        Err(e) => {
            return Json(resp::create(
                900,
                "get mongo fail".to_string(),
                Users {
                    name: e.to_string(),
                },
            ))
        }
    };

    return Json(resp::create(
        0,
        "success".to_string(),
        Users { name: res.concat() },
    ));
}
