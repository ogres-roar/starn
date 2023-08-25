use crate::data::StarnDB;
use crate::page::resp;
use crate::util::auth;
use rocket::{
    get, post,
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

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::NoSuchField(s) => write!(f, " {}", s),
            Self::NoSuchUser => write!(f, " no such user"),
            Self::ReadDbFail(s) => write!(f, " read db fail: {}", s),
        }
    }
}

fn remove_pswd(m: auth::User) -> auth::User {
    return auth::User {
        uid: m.uid,
        name: m.name,
        role: m.role,
        pswd: None,
    };
}

#[get("/user/<uid>")]
pub async fn get_user(cli: Connection<StarnDB>, uid: String) -> Json<resp::Resp<auth::User>> {
    let meta = match get_meta_from_db(&uid, &cli).await {
        Ok(m) => m,
        Err(e) => {
            return Json(resp::create(1000001, e.to_string(), None));
        }
    };

    return Json(resp::create(
        0,
        "success".to_string(),
        Some(remove_pswd(meta)),
    ));
}

async fn get_meta_from_db(uid: &str, cli: &Connection<StarnDB>) -> Result<auth::User, UserError> {
    let coll = cli.database("starn").collection::<Document>("user");

    match coll.find_one(doc! {"uid": uid}, None).await {
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

async fn delete_meta_from_db(uid: &str, cli: &Connection<StarnDB>) -> Result<(), UserError> {
    let coll = cli.database("starn").collection::<Document>("user");
    match coll.delete_one(doc! {"uid": uid}, None).await {
        Ok(d) => {
            if d.deleted_count == 0 {
                return Err(UserError::NoSuchUser);
            }
            return Ok(());
        }
        Err(e) => {
            return Err(UserError::ReadDbFail(e.to_string()));
        }
    }
}

fn get_meta_from_doc(d: Document) -> Result<auth::User, UserError> {
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

    return Ok(auth::User {
        uid: Some(uid),
        name: name,
        role: Some(role),
        pswd: Some(pswd),
    });
}

fn get_userinfo_from_doc(d: Document) -> Result<auth::User, UserError> {
    let m = match get_meta_from_doc(d) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    return Ok(remove_pswd(m));
}

async fn insert_meta_into_db(
    m: &auth::User,
    cli: &Connection<StarnDB>,
) -> Result<auth::User, UserError> {
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

#[post("/user/new", data = "<user>")]
pub async fn create(
    cli: Connection<StarnDB>,
    user: Json<auth::User>,
) -> Json<resp::Resp<auth::User>> {
    let uid = auth::generate_uid(&user.name);
    let _ = match get_meta_from_db(&uid, &cli).await {
        Ok(_) => {
            return Json(resp::create(1000011, "username exists".to_string(), None));
        }
        Err(UserError::NoSuchUser) => {}
        Err(e) => return Json(resp::create(1000012, e.to_string(), None)),
    };

    let pswd = match user.pswd {
        None => return Json(resp::create(1000014, "password empty".to_string(), None)),
        Some(ref p) => generate_pswd(p.as_str()),
    };

    let meta = match insert_meta_into_db(
        &auth::User {
            uid: Some(uid),
            name: user.name.clone(),
            role: user.role.to_owned(),
            pswd: Some(pswd),
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
        Some(remove_pswd(meta)),
    ));
}

#[get("/user/delete/<uid>")]
pub async fn delete(cli: Connection<StarnDB>, uid: String) -> Json<resp::Resp<Option<()>>> {
    match delete_meta_from_db(uid.as_str(), &cli).await {
        Ok(..) => {
            return Json(resp::create(0, "success".to_string(), None));
        }
        Err(e) => {
            return Json(resp::create(1000040, e.to_string(), None));
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Users {
    users: Vec<auth::User>,
    has_more: bool,
}

#[get("/users?<pn>")]
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
        Err(e) => return Json(resp::create(1000021, e.to_string(), None)),
    };

    let mut len = 0;
    let mut has_more = false;
    let mut users: Vec<auth::User> = vec![];

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
pub struct Jwt {
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt: Option<String>,
}

#[post("/login", data = "<user>")]
pub async fn login(cli: Connection<StarnDB>, user: Json<auth::User>) -> Json<resp::Resp<Jwt>> {
    let uid = auth::generate_uid(&user.name);

    let m = match get_meta_from_db(&uid, &cli).await {
        Ok(m) => m,
        Err(_) => {
            return Json(resp::create(1000030, "read from db fail".to_string(), None));
        }
    };

    let pswd = match user.pswd {
        None => return Json(resp::create(1000031, "password empty".to_string(), None)),
        Some(ref p) => p.to_owned(),
    };

    let dbpswd = match m.pswd {
        None => {
            return Json(resp::create(
                1000032,
                "pswd and name does not match".to_string(),
                None,
            ));
        }
        Some(ref p) => p.to_owned(),
    };

    if dbpswd != generate_pswd(pswd.as_str()) {
        return Json(resp::create(
            1000033,
            "pswd and name does not match".to_string(),
            None,
        ));
    }

    let role = match m.role {
        None => {
            return Json(resp::create(1000032, "no role in db".to_string(), None));
        }
        Some(r) => r,
    };

    return Json(resp::create(
        0,
        "success".to_string(),
        Some(Jwt {
            jwt: auth::generate_default_jwt(user.name.to_string(), role),
        }),
    ));
}

#[get("/logout")]
pub async fn logout() -> Json<resp::Resp<Option<auth::User>>> {
    return Json(resp::create(0, "success".to_string(), None));
}

fn generate_pswd(pswd: &str) -> String {
    let pswd = String::from("pass--") + pswd + "--word";
    let mut state = DefaultHasher::new();
    pswd.hash(&mut state);
    return state.finish().to_string();
}
