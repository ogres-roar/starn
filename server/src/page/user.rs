use crate::data::StarnDB;
use crate::page::resp;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use rocket_db_pools::mongodb::{
    bson::{doc, Document},
    options::FindOptions,
};
use rocket_db_pools::Connection;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum ParseError {
    NoSuchField(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NoSuchField(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct UserInfo {
    uid: String,
    name: String,
    role: String,
}

#[get("/starn/user/<uid>")]
pub async fn get_user(cli: Connection<StarnDB>, uid: String) -> Json<resp::Resp<Option<UserInfo>>> {
    let coll = cli.database("starn").collection::<Document>("user");

    let _ = match coll.find_one(doc! {"uid": uid}, None).await {
        Ok(d) => {
            let _ = match d {
                None => {
                    return Json(resp::create(1000001, "no such user".to_string(), None));
                }
                Some(u) => {
                    let _ = match get_userinfo_from_doc(u) {
                        Ok(ui) => {
                            return Json(resp::create(0, "success".to_string(), Some(ui)));
                        }
                        Err(e) => {
                            return Json(resp::create(1000003, e.to_string(), None));
                        }
                    };
                }
            };
        }
        Err(e) => {
            return Json(resp::create(1000002, e.to_string(), None));
        }
    };
}

fn get_userinfo_from_doc(d: Document) -> Result<UserInfo, ParseError> {
    let uid = match d.get_str("uid") {
        Err(e) => return Err(ParseError::NoSuchField(e.to_string())),
        Ok(u) => u.to_string(),
    };

    let name = match d.get_str("name") {
        Err(e) => return Err(ParseError::NoSuchField(e.to_string())),
        Ok(u) => u.to_string(),
    };

    let role = match d.get_str("role") {
        Err(e) => return Err(ParseError::NoSuchField(e.to_string())),
        Ok(u) => u.to_string(),
    };
    return Ok(UserInfo {
        uid: uid,
        name: name,
        role: role,
    });
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct User {
    name: String,
    pswd: String,
    role: String,
}
#[post("/starn/user/new", data = "<user>")]
pub async fn create_user(
    cli: Connection<StarnDB>,
    user: Json<User>,
) -> Json<resp::Resp<Option<UserInfo>>> {
    let coll = cli.database("starn").collection::<Document>("user");
    let mut state = DefaultHasher::new();
    user.name.hash(&mut state);
    let uid = state.finish().to_string();

    let _ = match coll.find_one(doc! {"uid": &uid}, None).await {
        Ok(d) => {
            let _ = match d {
                None => ..,
                Some(_) => {
                    return Json(resp::create(1000011, "username exists".to_string(), None));
                }
            };
        }
        Err(e) => return Json(resp::create(1000012, e.to_string(), None)),
    };

    let _ = match coll
        .insert_one(
            doc! {"uid": &uid, "name":&user.name, "pswd":&user.pswd, "role":&user.role},
            None,
        )
        .await
    {
        Ok(_) => ..,
        Err(e) => {
            return Json(resp::create(1000013, e.to_string(), None));
        }
    };
    return Json(resp::create(
        0,
        "success".to_string(),
        Some(UserInfo {
            uid: uid,
            name: user.name.to_string(),
            role: user.role.to_string(),
        }),
    ));
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Users {
    users: Vec<UserInfo>,
    has_more: bool,
}

#[get("/starn/users?<pn>")]
pub async fn get_users(cli: Connection<StarnDB>, mut pn: u64) -> Json<resp::Resp<Users>> {
    let coll = cli.database("starn").collection::<Document>("user");
    if pn > 0 {
        pn = pn - 1;
    }

    let ps: i64 = 20;
    let opt = FindOptions::builder()
        .skip(ps as u64 * pn)
        .limit(ps + (1 as i64))
        .build();
    let mut cursor = match coll.find(doc! {}, opt).await {
        Ok(c) => c,
        Err(e) => {
            return Json(resp::create(
                1000021,
                e.to_string(),
                Users {
                    users: vec![],
                    has_more: false,
                },
            ))
        }
    };

    let mut len = 0;
    let mut has_more = false;
    let mut users: Vec<UserInfo> = vec![];

    while match cursor.advance().await {
        Ok(exists) => exists,
        Err(e) => {
            return Json(resp::create(
                1000022,
                e.to_string(),
                Users {
                    users: vec![],
                    has_more: false,
                },
            ));
        }
    } {
        let d = match cursor.deserialize_current() {
            Ok(d) => d,
            Err(e) => {
                return Json(resp::create(
                    1000022,
                    e.to_string(),
                    Users {
                        users: vec![],
                        has_more: false,
                    },
                ));
            }
        };
        len += 1;
        if len == 21 {
            has_more = true;
            break;
        }
        let d = match get_userinfo_from_doc(d) {
            Ok(d) => d,
            Err(e) => {
                return Json(resp::create(
                    1000023,
                    e.to_string(),
                    Users {
                        users: vec![],
                        has_more: false,
                    },
                ));
            }
        };
        users.push(d);
    }

    return Json(resp::create(
        0,
        "success".to_string(),
        Users {
            users: users,
            has_more: has_more,
        },
    ));
}
