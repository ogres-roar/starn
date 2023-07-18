use super::success;
use rocket::post;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{info, warn};

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
    info!("tttttt info");
    warn!("tttttt warn");
    return Json(success::create(user.into_inner() as User));
}
