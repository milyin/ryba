use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use ryba_kit::auth::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use pages::*;
use rocket::State;
use std::sync::Mutex;
use Users;

#[derive(FromForm)]
pub struct Register<'a> {
    name: Field<'a, String>,
    age: Field<'a, Age>,
    password: Field<'a, String>,
    password1: Field<'a, String>,
}

#[derive(Serialize,Default)]
pub struct RegisterContext {
    name: ContextField<String>,
    age: ContextField<Age>,
    password: ContextField<String>,
    password1: ContextField<String>,
}

impl<'a> Register<'a> {
    fn context(&'a self) -> RegisterContext {
        RegisterContext {
            name: (&self.name).into(),
            age: (&self.age).into(),
            password: (&self.password).into(),
            password1: (&self.password1).into(),
        }
    }
    fn values(&'a self) -> Option<(&'a String, &'a Age, &'a String, &'a String)> {
        if let (&Ok(ref name), &Ok(ref age), &Ok(ref password), &Ok(ref password1)) = 
            (&self.name.value, &self.age.value, &self.password.value, &self.password1.value) {
            Some((name,age,password,password1))
        }
        else
        {
            None
        }
    }
}

#[derive(Serialize)]
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

#[derive(Serialize, Default, Copy, Clone)]
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
fn post<'a>(mut _users: State<Mutex<Users>>,
            mut ctx: Context<Page>,
            data: Form<'a, Register<'a>>)
            -> Result<Redirect, Template>
{
    let mut users = match _users.inner().lock() {
        Ok(users) => users,
        Err(e) => return Ok(Redirect::to("/error"))
    };

    let form = data.get();
    ctx.page.form = form.context();

    if let Some((name,age,password,password1)) = form.values() {
        if password != password1 {
            ctx.page.form.password.msg = Some("password not match".to_string())
        }
        if users.get(name) != None {
            ctx.page.form.name.msg = Some("user exists".to_string());
        } else {
            users.insert(name.clone(), password.clone());
            return Ok(Redirect::to("/"));
        }
    }

    Err(Template::render("register", ctx))
}
