use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use users::*;

#[derive(FromForm,ToContext)]
struct Register<'a> {
    name: Field<'a, String>,
    age: Field<'a, Age>,
    password: Field<'a, String>,
    password1: Field<'a, String>,
}

#[derive(Serialize,Debug)]
pub struct Page {
    title: &'static str,
    form: RegisterContext,
}

impl Default for Page {
    fn default() -> Page {
        Self {
            title: "Registration",
            form: RegisterContext::default(),
        }
    }
}

#[derive(Serialize, Default, Copy, Clone, Debug)]
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
fn post<'a>(mut ctx: Context<Page>, data: Form<'a, Register<'a>>) -> Result<Redirect, Template> {
    let form = data.get();
    ctx.page.form = form.context();

    if let Some((name, _, password, password1)) = form.values() {
        if password != password1 {
            ctx.page.form.password.msg = Some("password not match".to_string())
        } else if let Err(msg) = add_user(name, password) {
            ctx.page.form.name.msg = Some(msg);
        } else {
            return Ok(Redirect::to("/"));
        }
    }

    Err(Template::render("register", ctx))
}
