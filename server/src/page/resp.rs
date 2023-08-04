use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Resp<T> {
    errno: u32,
    msg: String,
    data: T,
}

pub fn create<T>(no: u32, msg: String, data: T) -> Resp<T> {
    return Resp {
        errno: no,
        msg: msg,
        data: data,
    };
}
