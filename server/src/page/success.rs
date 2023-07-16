use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Success<T> {
    errno: u16,
    msg: String,
    data: T,
}

pub fn create<T>(data: T) -> Success<T> {
    return Success {
        errno: 0,
        msg: "success".to_string(),
        data: data,
    };
}
