use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Resp<T> {
    errno: u32,
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

pub fn create<T>(no: u32, msg: String, data: Option<T>) -> Resp<T> {
    return Resp {
        errno: no,
        msg: msg,
        data: data,
    };
}
