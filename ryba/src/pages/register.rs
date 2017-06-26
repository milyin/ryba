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

trait RegisterFields<'a> {
    fn name(&'a self) -> &'a String;
    fn age(&'a self) -> &'a Age;
    fn password(&'a self) -> &'a String;
    fn password1(&'a self) -> &'a String;
}

trait FieldForm2<'a> {
    fn data(&'a self) -> Option<&'a RegisterFields<'a>>;
}

struct Proxy<'a, T>(&'a T) where T: 'a;

impl<'a> RegisterFields<'a> for Proxy<'a, Register> {
    fn name(&'a self) -> &'a String {
        self.0.name.get().unwrap()
    }
    fn age(&'a self) -> &'a Age {
        self.0.age.get().unwrap()
    }
    fn password(&'a self) -> &'a String {
        self.0.password.get().unwrap()
    }
    fn password1(&'a self) -> &'a String {
        self.0.password1.get().unwrap()
    }
}

impl<'a> FieldForm2<'a> for Register {
    fn data(&'a self) -> Option<&'a RegisterFields<'a>> {
        if self.name.is_ok() {
            Some(&Proxy(self))
        } else {
            None
        }
    }
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
fn post<'a>(users: State<Users>,
            mut ctx: Context<Page>,
            data: Form<'a, Register>)
            -> Result<Redirect, Template> {
    let mut form = data.into_inner();
    if form.password.get() != form.password1.get() {
        form.password1.set_msg("password not match".to_string());
    }

    if 

    if users.get(form.name.get()) != None {
        form.name.set_msg("user exists".to_string());
    }
    if form.is_err() || form.has_msg() {
        ctx.page.form = form;
        Err(Template::render("register", ctx))
    } else {
        users.insert(form.name.get(),
                     Session::hash(form.name.get(), ctx.session.extra_data, form.password.get()));
        Ok(Redirect::to("/"))
    }
}
