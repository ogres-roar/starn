use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Failure<T> {
    errno: u16,
    msg: String,
    data: T,
}

pub fn create<T>(errno: u16, msg: String, data: T) -> Failure<T> {
    return Failure {
        errno: errno,
        msg: msg,
        data: data,
    };
}
