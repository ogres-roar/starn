/// 数据层
use rocket_db_pools::{mongodb, Database};

#[derive(Database)]
#[database("starn")]
pub struct StarnDB(mongodb::Client);

impl StarnDB {
    pub fn setup() -> rocket_db_pools::Initializer<Self> {
        StarnDB::init()
    }
}
