mod data;
mod page;
mod util;
use figment::providers::{Format, Toml};
use page::{catcher, user};
use rocket::{
    catchers,
    fs::{relative, FileServer},
    launch, routes,
};

#[launch]
fn rocket() -> _ {
    util::log::init_logger();

    // rocket 配置
    let figment = rocket::Config::figment().merge(Toml::file("conf/rocket.toml").nested());
    rocket::custom(figment)
        .register(
            "/",
            catchers![catcher::default_catcher, catcher::fail_guard],
        )
        .attach(data::StarnDB::setup())
        .attach(util::request::Context {})
        .mount(
            "/starn",
            routes![
                user::create,
                user::delete,
                user::get_user,
                user::get_users,
                user::login,
                user::logout,
            ],
        )
        .mount("/", FileServer::from(relative!("website")))
}
