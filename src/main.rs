#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

extern crate rand;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

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
use state::global_config::GlobalConfig;

#[get("/")]
fn index() -> &'static str {
    "Hello Rocket !"
}

fn main() {
    println!("Loading config ...");
    let config: GlobalConfig = GlobalConfig::load();
    println!("Config loaded successfully !");

    println!("Launching the server ...");
    rocket::ignite()
        .manage(config)
        .attach(DatabaseConn::fairing())
        .mount("/", routes![index])
        .mount("/login", routes![login::github::cb_login_github])
        .launch();
}
