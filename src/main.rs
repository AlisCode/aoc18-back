#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

extern crate rand;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate toml;

pub mod db;
pub mod login;
pub mod model;
pub mod schema;
pub mod state;

use db::DatabaseConn;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use state::global_config::GlobalConfig;

#[get("/")]
fn index() -> &'static str {
    "Hello Rocket !"
}

fn main() {
    // Load config
    println!("Loading config ...");
    let config: GlobalConfig = GlobalConfig::load();
    println!("Config loaded successfully !");

    // Setup CORS Options
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&[
        "http://192.168.1.1:8080",
        "http://192.168.1.1:8000",
        "http://localhost",
    ]);
    let cors_options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    println!("Launching the server ...");
    rocket::ignite()
        .manage(config)
        .attach(DatabaseConn::fairing())
        .attach(cors_options)
        .mount("/", routes![index])
        .mount(
            "/login",
            routes![
                login::github::cb_login_github,
                login::gitlab::cb_login_gitlab
            ],
        )
        .mount("/api", routes![model::user::get_username])
        .launch();
}
