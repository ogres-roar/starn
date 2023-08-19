/// 用户 角色 权限
use carte::auth::role::Role;

#[derive(Debug)]
pub struct User {
    name: String,
    uid: String,
    passwd: String,
    role: Role,
}

pub fn salt(passwd: &str) -> String {
    return format!(
        "{:x}",
        md5::compute(String::from("pass-") + passwd + "-world")
    );
}

impl User {
    pub fn create(name: String, passwd: String, role: Role) -> User {
        return User {
            name: name.clone(),
            uid: format!("{:x}", md5::compute(name)),
            passwd: passwd,
            role: role,
        };
    }
}
