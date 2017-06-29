#![feature(plugin, custom_derive, use_extern_macros, decl_macro)]
#![plugin(rocket_codegen)]

extern crate ryba_kit;
extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate ryba_kit_derive;

use std::sync::Mutex;
type Users = HashMap<String,String>;

mod context;
mod pages;

use ryba_kit::template::*;
use ryba_kit::helpers::*;
use ryba_kit::auth::*;
use std::collections::HashMap;
use pages::*;

fn main() {
    init_handlebars(add_helpers);
    add_templates("templates").expect("Failed to read templates");
    rocket::ignite()
        .mount("/", routes![
            index::get,
            register::get, register::post,
            login::get, login::post
        ])
        .manage(Mutex::new(Users::new()))
        .launch();
}
