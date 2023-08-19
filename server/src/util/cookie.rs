use rocket::http::{Cookie, CookieJar};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn generate_validate_code(uid: &str, role: &str) -> String {
    let pswd = String::from("validate-#uid") + uid + "--#role#--" + role + "#-code";
    let mut state = DefaultHasher::new();
    pswd.hash(&mut state);
    return state.finish().to_string();
}

pub fn is_login(cookies: &CookieJar<'_>) -> bool {
    let uid = match cookies.get("uid") {
        None => {
            clear_cookie(cookies);
            return false;
        }
        Some(uid) => uid.value(),
    };
    let role = match cookies.get("role") {
        None => {
            clear_cookie(cookies);
            return false;
        }
        Some(role) => role.value(),
    };
    let usd = match cookies.get("usd") {
        None => {
            clear_cookie(cookies);
            return false;
        }
        Some(usd) => usd.value(),
    };
    let res = generate_validate_code(uid, role).eq(usd);
    if !res {
        clear_cookie(cookies);
    }
    return res;
}

pub fn clear_cookie(cookies: &CookieJar<'_>) {
    cookies.remove(Cookie::named("uid"));
    cookies.remove(Cookie::named("role"));
    cookies.remove(Cookie::named("usd"));
}
