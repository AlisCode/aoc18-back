#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate rand;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod db;
pub mod model;
pub mod schema;
