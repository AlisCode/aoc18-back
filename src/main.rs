#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate toml;

use state::global_config::GlobalConfig;

pub mod login;
pub mod state;

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
        .mount("/", routes![index])
        .mount("/login", routes![login::github::cb_login_github])
        .launch();
}
