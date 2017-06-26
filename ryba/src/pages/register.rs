use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use ryba_kit::auth::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use pages::*;
use rocket::State;
use Users;

#[derive(Serialize,FromForm,Default,FieldForm)]
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
fn post<'a>(mut users: State<Users>,
            mut ctx: Context<Page>,
            data: Form<'a, Register>)
            -> Result<Redirect, Template> {
    let mut form = data.into_inner();
    if form.password.get() != form.password1.get() {
        form.password1.set_msg("password not match".to_string());
    }
    if form.is_ok() && !form.has_msg() {
        if let (&Ok(ref name), &Ok(ref password)) = (&form.name.value, &form.password.value) {
            if users.get(name) != None {
                form.name.msg = Some("user exists".to_string());
            } else {
                //users.insert(name.clone(), password.clone());
                return Ok(Redirect::to("/"));
            }
        }
    }
    ctx.page.form = form;
    Err(Template::render("register", ctx))
}
