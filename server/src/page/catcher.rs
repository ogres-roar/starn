use rocket::catch;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Unexpection {
    errno: u32,
    msg: String,
}

#[catch(default)]
pub fn default_catcher(_: Status, _: &Request) -> Json<Unexpection> {
    return Json(Unexpection {
        errno: 90000,
        msg: "default error catcher".to_string(),
    });
}

#[catch(422)]
pub fn fail_guard(_: &Request) -> Json<Unexpection> {
    return Json(Unexpection {
        errno: 90001,
        msg: "parse input fail".to_string(),
    });
}
