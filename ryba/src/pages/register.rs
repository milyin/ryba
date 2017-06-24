use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;

#[derive(Serialize,FromForm,Default)]
struct Register {
    name: Field<String>,
    age: Field<Age>,
    password: Field<String>,
    password1: Field<String>,
}

#[derive(Serialize)]
struct Page {
    title: &'static str,
    form: Register,
}

impl Default for Page {
    fn default() -> Page {
        Self {
            title: "Registration",
            form: Register::default(),
        }
    }
}

#[derive(Serialize, Default)]
struct Age(usize);

impl<'v> FromFormValue<'v> for Age {
    type Error = &'v str;
    fn from_form_value(form_value: &'v str) -> Result<Self, Self::Error> {
        let v = usize::from_form_value(form_value);
        match v {
            Ok(age) => {
                if age < 21 {
                    Err("too young")
                } else {
                    Ok(Age(age))
                }
            }
            Err(_) => Err("parse error"),
        }
    }
}

#[get("/register")]
pub fn get(req: Req) -> Template {
    let ctx = Context::new(req, Page::default());
    Template::render("register", &ctx)
}

#[post("/register", data="<data>")]
fn post<'a>(req: Req, data: Form<'a, Register>) -> Result<Redirect, Template> {
    let mut form = data.into_inner();
    if form.password.get() != form.password1.get() {
        form.password1.set_msg("password not match".to_string());
    }
    let ctx = Context::new(req,
                           Page {
                               form: form,
                               ..Page::default()
                           });
    Err(Template::render("register", &ctx))
}
