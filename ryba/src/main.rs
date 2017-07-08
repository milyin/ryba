#![feature(plugin, custom_derive, use_extern_macros, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate ryba_kit;
extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate ryba_kit_derive;

mod context;
mod pages;
mod users;

use ryba_kit::template::*;
use ryba_kit::helpers::*;
use pages::*;

fn main() {
    init_handlebars(add_helpers);
    add_templates("templates").expect("Failed to read templates");
    rocket::ignite()
        .mount("/",
               routes![index::get,
                       register::get,
                       register::post,
                       login::get,
                       login::post_login,
                       login::post_logout])
        .launch();
}
