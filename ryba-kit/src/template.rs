use handlebars::Handlebars;
use std::path::{Path, PathBuf};
use glob::glob;
use std::sync::Mutex;
use itertools::Itertools;
use std::error::Error;
use std::borrow::Cow;
use serde_json::{Value, to_value};
use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::ContentType;
use rocket::http::Status;
use serde::ser::Serialize;
use std::io::Cursor;

#[derive(Debug)]
pub struct Template {
    name: Cow<'static, str>,
    value: Option<Value>,
}

lazy_static! {
    static ref HANDLEBARS: Mutex<Handlebars> = Mutex::new(Handlebars::new());
}

pub fn init_handlebars(f: fn(&mut Handlebars)) {
    let mut hb = HANDLEBARS.lock().unwrap();
    f(&mut hb)
}

pub fn add_templates<P>(root: P) -> Result<(), Box<Error>>
    where P: Into<PathBuf>
{
    let mut hb = HANDLEBARS.lock().unwrap();
    let root_buf = root.into();
    let mut mask_buf = root_buf.clone();
    mask_buf.push("**");
    mask_buf.push("*.hbs");
    let mask = mask_buf.to_str().ok_or("read error")?;

    let add_template = &mut |entry: &Path| -> Result<(), Box<Error>> {
        let stripped = entry.strip_prefix(&root_buf)?.with_extension(""); // strip prefix and .hbs
        //let ext = stripped.extension().ok_or("no type extension")?; // skip if no .html or smth else
        let name: String = stripped
            .with_extension("")
            .to_str()
            .ok_or("can't convert path to string")?
            .chars()
            .filter_map(|c| Some(if c == '\\' { '/' } else { c }))
            .collect();
        println!("{}", &name);
        if let Err(e) = hb.register_template_file(&name, &entry) {
            // TODO: make correct error loagging
            println!("{} {}", &name, &e);
            error!("Error in Handlebars template {}", &name);
            info!("{}", e);
            info!("Template path: '{}'", entry.to_string_lossy());
        }
        Ok(())
    };

    glob(mask)
        .unwrap()
        .filter_map(Result::ok)
        .foreach(|entry| { let _ = add_template(&entry); });

    Result::Ok(())
}

impl Template {
    pub fn render<S, C>(name: S, context: C) -> Template
        where S: Into<Cow<'static, str>>,
              C: Serialize
    {
        Template {
            name: name.into(),
            value: to_value(context).ok(),
        }
    }
}

impl Responder<'static> for Template {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        let hb = HANDLEBARS.lock().unwrap();
        let render = hb.render(&self.name, &self.value).unwrap_or_else(|e| e.to_string());
        Response::build()
            .header(ContentType::HTML)
            .sized_body(Cursor::new(render))
            .ok()
    }
}