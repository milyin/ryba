use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use pages::*;

#[derive(Serialize,FromForm,Default)]
pub struct Register {
    name: Field<String>,
    age: Field<Age>,
    password: Field<String>,
    password1: Field<String>,
}

#[derive(Serialize)]
pub struct Page {
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
pub fn get(ctx: Context<Page>) -> Template {
    Template::render("register", &ctx)
}

#[post("/register", data="<data>")]
fn post<'a>(mut ctx: Context<Page>, data: Form<'a, Register>) -> Result<Redirect, Template> {
    let mut form = data.into_inner();
    if form.password.get() != form.password1.get() {
        form.password1.set_msg("password not match".to_string());
    }
    ctx.page.form = form;
    Err(Template::render("register", ctx))
}
