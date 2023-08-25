use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use log::warn;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const KEY: &[u8] = b"starn_magic_auth";

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    user: String,
    role: String,
    exp: u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pswd: Option<String>,
}

pub fn generate_uid(name: &str) -> String {
    let mut state = DefaultHasher::new();
    let name = String::from("user--") + name + "--name";
    name.hash(&mut state);
    return state.finish().to_string();
}

// Bearer Token
impl User {
    fn from_request(header: &str) -> Option<User> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        if split_vec[0] != "Bearer" {
            return None;
        }
        Self::from_jwt(split_vec[1])
    }

    fn from_jwt(token_string: &str) -> Option<User> {
        let val = Validation::default();
        match decode::<Claims>(token_string, &DecodingKey::from_secret(KEY), &val) {
            Ok(c) => {
                if c.claims.exp < get_current_timestamp() {
                    return None;
                }
                return Some(User {
                    uid: Some(generate_uid(&c.claims.user)),
                    name: c.claims.user,
                    role: Some(c.claims.role),
                    pswd: None,
                });
            }
            Err(_) => None,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_request(header_auth) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

pub fn generate_default_jwt(name: String, role: String) -> Option<String> {
    return generate_jwt(name, role, get_current_timestamp() + 60 * 60 * 24 * 7);
}

pub fn generate_jwt(name: String, role: String, expire: u64) -> Option<String> {
    let header = Header::default();
    let claims = Claims {
        user: name,
        role: role,
        exp: expire,
    };
    match encode(&header, &claims, &EncodingKey::from_secret(KEY)) {
        Err(e) => {
            warn!(target:"starn", "encode : {}", e);
            return None;
        }
        Ok(s) => {
            return Some(s);
        }
    }
}
