mod page;
use figment::providers::{Format, Toml};
use page::{catcher, user};
use rocket::{catchers, launch, routes};

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(Toml::file("conf/Rocket.toml").nested());
    rocket::custom(figment)
        .register(
            "/",
            catchers![catcher::default_catcher, catcher::fail_guard],
        )
        .mount("/", routes![user::create_user])
}
