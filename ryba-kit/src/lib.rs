#![feature(decl_macro, never_type)]
#![crate_type = "proc-macro"]

extern crate glob;
extern crate handlebars;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rocket;
extern crate serde;
extern crate serde_json;

pub mod form;
pub mod template;
pub mod helpers;
pub mod auth;
