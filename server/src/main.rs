#[macro_use]
extern crate rocket;

use rocket::{
    data::{Limits, ToByteUnit},
    http::ContentType,
    Config, Request,
};

#[get("/")]
fn index() -> (ContentType, &'static str) {
    (
        ContentType::HTML,
        include_str!("../../client/static/index.html"),
    )
}

#[get("/style.css")]
fn style() -> (ContentType, &'static str) {
    (
        ContentType::CSS,
        include_str!("../../client/static/style.css"),
    )
}

#[get("/seed_rs_base.js")]
fn script() -> (ContentType, &'static str) {
    (
        ContentType::JavaScript,
        include_str!("../../client/pkg/seed_rs_base.js"),
    )
}

#[get("/seed_rs_base_bg.wasm")]
fn wasm() -> (ContentType, &'static [u8]) {
    (
        ContentType::WASM,
        include_bytes!("../../client/pkg/seed_rs_base_bg.wasm"),
    )
}

#[get("/favicon.png")]
fn favicon() -> (ContentType, &'static [u8]) {
    (
        ContentType::Icon,
        include_bytes!("../../client/static/favicon.png"),
    )
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[tokio::main]
async fn main() {
    let app = rocket::build()
        .configure(Config {
            log_level: rocket::config::LogLevel::Debug,
            workers: 16,
            limits: Limits::new().limit("json", 10.gigabytes()),
            ..Config::release_default()
        })
        .mount("/", routes![index, style, script, wasm, favicon])
        .register("/", catchers![not_found])
        .launch();
    app.await.unwrap();
}
