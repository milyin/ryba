extern crate serde;
extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate handlebars;
#[macro_use] extern crate lazy_static;
extern crate glob;
#[macro_use] extern crate itertools;
#[macro_use] extern crate log;
extern crate serde_json;

pub mod form;
pub mod context;
pub mod template;
pub mod helpers;