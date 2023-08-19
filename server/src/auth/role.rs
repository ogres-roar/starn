/// 角色
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Action {
    NONE,  // 无权限
    READ,  // 只读
    WRITE, // 只写
}

#[derive(Debug)]
pub type Sections = Vec<String>;

#[derive(Debug)]
pub struct Chapter {
    name: String,       // 当前目录名称
    sections: Sections, // 目录结构
}

pub type Catalog = HashMap<String, Chapter>;

#[derive(Debug)]
pub struct Privilege {
    sections: Section, // 目录路径
    action: Action,    // 权限
}

#[derive(Debug)]
pub struct Role {
    rid: String,                            // md5(name)
    name: String,                           // 角色名称
    privileges: HashMap<String, Privilege>, // 路径权限
}

impl Role {
    pub fn create(name: String, privileges: HashMap<String, Privilege>) -> Role {
        return Role {
            name: name.clone(),
            rid: format!("{:x}", md5::compute(name)),
            privileges: privileges,
        };
    }

    pub fn can_read(&self, catalog: &str, page: &str) -> bool {
        match self.privileges.get(catalog) {
            Some(privilege) => {
                if privilege.action == Action::NONE {
                    return false;
                }
                for ref _page in privilege.catalog.children.clone() {
                    if page.eq(_page) {
                        return true;
                    }
                }
                return false;
            }
            _ => return false,
        }
    }

    pub fn can_write(&self, catalog: &str, page: &str) -> bool {
        match self.privileges.get(catalog) {
            Some(privilege) => {
                if privilege.action != Action::WRITE {
                    return false;
                }
                for ref _page in privilege.catalog.children.clone() {
                    if page.eq(_page) {
                        return true;
                    }
                }
                return false;
            }
            _ => return false,
        }
    }
}
