use crate::data::StarnDB;
use crate::page::resp;

use crate::util::cookie;
use rocket::{
    get,
    http::{Cookie, CookieJar},
    post,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::{
    mongodb::{
        bson::{doc, Document},
        options::FindOptions,
    },
    Connection,
};
use std::{
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
pub enum UserError {
    NoSuchUser,
    NoSuchField(String),
    ReadDbFail(String),
}

#[derive(Clone)]
pub struct Meta {
    uid: String,
    name: String,
    role: String,
    pswd: String,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::NoSuchField(s) => write!(f, " {}", s),
            Self::NoSuchUser => write!(f, " no such user"),
            Self::ReadDbFail(s) => write!(f, " read db fail: {}", s),
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

fn get_userinfo(m: Meta) -> UserInfo {
    return UserInfo {
        uid: m.uid,
        name: m.name,
        role: m.role,
    };
}

#[get("/user/<uid>")]
pub async fn get_user(cli: Connection<StarnDB>, uid: String) -> Json<resp::Resp<Option<UserInfo>>> {
    let meta = match get_meta_from_db(&uid, &cli).await {
        Ok(m) => m,
        Err(e) => {
            return Json(resp::create(1000001, e.to_string(), None));
        }
    };

    return Json(resp::create(
        0,
        "success".to_string(),
        Some(get_userinfo(meta)),
    ));
}

async fn get_meta_from_db(uid: &str, cli: &Connection<StarnDB>) -> Result<Meta, UserError> {
    let coll = cli.database("starn").collection::<Document>("user");

    let _ = match coll.find_one(doc! {"uid": uid}, None).await {
        Ok(d) => {
            let _ = match d {
                None => {
                    return Err(UserError::NoSuchUser);
                }
                Some(u) => {
                    let _ = match get_meta_from_doc(u) {
                        Ok(ui) => {
                            return Ok(ui);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                }
            };
        }
        Err(e) => {
            return Err(UserError::ReadDbFail(e.to_string()));
        }
    };
}

fn get_meta_from_doc(d: Document) -> Result<Meta, UserError> {
    let uid = match d.get_str("uid") {
        Err(e) => return Err(UserError::NoSuchField(e.to_string())),
        Ok(u) => u.to_string(),
    };

    let name = match d.get_str("name") {
        Err(e) => return Err(UserError::NoSuchField(e.to_string())),
        Ok(u) => u.to_string(),
    };

    let role = match d.get_str("role") {
        Err(e) => return Err(UserError::NoSuchField(e.to_string())),
        Ok(u) => u.to_string(),
    };

    let pswd = match d.get_str("pswd") {
        Err(e) => return Err(UserError::NoSuchField(e.to_string())),
        Ok(p) => p.to_string(),
    };

    return Ok(Meta {
        uid: uid,
        name: name,
        pswd: pswd,
        role: role,
    });
}

fn get_userinfo_from_doc(d: Document) -> Result<UserInfo, UserError> {
    let m = match get_meta_from_doc(d) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    return Ok(get_userinfo(m));
}

async fn insert_meta_into_db(m: &Meta, cli: &Connection<StarnDB>) -> Result<Meta, UserError> {
    let coll = cli.database("starn").collection::<Document>("user");
    let _ = match coll
        .insert_one(
            doc! {"uid": &m.uid, "name":&m.name, "pswd":&m.pswd, "role":&m.role},
            None,
        )
        .await
    {
        Ok(_) => return Ok(m.clone()),
        Err(e) => {
            return Err(UserError::ReadDbFail(e.to_string()));
        }
    };
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct User {
    name: String,
    pswd: String,
    role: String,
}

#[post("/user/new", data = "<user>")]
pub async fn create_user(
    cli: Connection<StarnDB>,
    user: Json<User>,
    cookies: &CookieJar<'_>,
) -> Json<resp::Resp<Option<UserInfo>>> {
    if !cookie::is_login(cookies) {
        return Json(resp::create(1000010, "pleas login".to_string(), None));
    }
    let uid = generate_uid(&user.name);
    let _ = match get_meta_from_db(&uid, &cli).await {
        Ok(_) => {
            return Json(resp::create(1000011, "username exists".to_string(), None));
        }
        Err(UserError::NoSuchUser) => {}
        Err(e) => return Json(resp::create(1000012, e.to_string(), None)),
    };

    let coll = cli.database("starn").collection::<Document>("user");
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

    let meta = match insert_meta_into_db(
        &Meta {
            uid: uid,
            name: user.name.clone(),
            role: user.role.clone(),
            pswd: generate_pswd(&user.pswd),
        },
        &cli,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Json(resp::create(1000014, e.to_string(), None)),
    };
    return Json(resp::create(
        0,
        "success".to_string(),
        Some(get_userinfo(meta)),
    ));
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Users {
    users: Vec<UserInfo>,
    has_more: bool,
}

#[get("/users?<pn>")]
pub async fn get_users(cli: Connection<StarnDB>, mut pn: u64) -> Json<resp::Resp<Option<Users>>> {
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
        Err(e) => return Json(resp::create(1000021, e.to_string(), None)),
    };

    let mut len = 0;
    let mut has_more = false;
    let mut users: Vec<UserInfo> = vec![];

    while match cursor.advance().await {
        Ok(exists) => exists,
        Err(e) => {
            return Json(resp::create(1000022, e.to_string(), None));
        }
    } {
        let d = match cursor.deserialize_current() {
            Ok(d) => d,
            Err(e) => {
                return Json(resp::create(1000022, e.to_string(), None));
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
                return Json(resp::create(1000023, e.to_string(), None));
            }
        };
        users.push(d);
    }

    return Json(resp::create(
        0,
        "success".to_string(),
        Some(Users {
            users: users,
            has_more: has_more,
        }),
    ));
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct Login {
    name: String,
    pswd: String,
}

#[post("/login", data = "<user>")]
pub async fn login(
    cli: Connection<StarnDB>,
    user: Json<Login>,
    cookies: &CookieJar<'_>,
) -> Json<resp::Resp<Option<UserInfo>>> {
    let uid = generate_uid(&user.name);

    let m = match get_meta_from_db(&uid, &cli).await {
        Ok(m) => m,
        Err(_) => {
            cookie::clear_cookie(cookies);
            return Json(resp::create(
                1000030,
                "pswd and name does not match".to_string(),
                None,
            ));
        }
    };

    if m.pswd != generate_pswd(&user.pswd) {
        cookie::clear_cookie(cookies);
        return Json(resp::create(
            1000030,
            "pswd and name does not match".to_string(),
            None,
        ));
    }

    set_login_info(&uid, &m.role, cookies);

    return Json(resp::create(
        0,
        "success".to_string(),
        Some(get_userinfo(m)),
    ));
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Json<resp::Resp<Option<UserInfo>>> {
    cookie::clear_cookie(cookies);
    return Json(resp::create(0, "success".to_string(), None));
}

fn generate_uid(name: &str) -> String {
    let mut state = DefaultHasher::new();
    let name = String::from("user--") + name + "--name";
    name.hash(&mut state);
    return state.finish().to_string();
}

fn generate_pswd(pswd: &str) -> String {
    let pswd = String::from("pass--") + pswd + "--word";
    let mut state = DefaultHasher::new();
    pswd.hash(&mut state);
    return state.finish().to_string();
}

fn set_login_info(uid: &str, role: &str, cookies: &CookieJar<'_>) {
    cookies.add(Cookie::new("uid", uid.to_string()));
    cookies.add(Cookie::new("role", role.to_string()));
    cookies.add(Cookie::new(
        "usd",
        cookie::generate_validate_code(&uid, &role),
    ));
}
