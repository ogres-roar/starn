/// mongo 接口
use rocket_db_pools::{mongodb, Database};

#[derive(Database)]
#[database("starn")]
struct StarnDB(mongodb::Client);
