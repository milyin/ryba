use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use ryba_kit::auth::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket::http::Cookies;
use rocket::State;
use pages::*;
use Users;

#[derive(Serialize)]
pub struct Page {
    title: &'static str,
}

impl Default for Page {
    fn default() -> Page {
        Self { title: "Login" }
    }
}

#[get("/login")]
pub fn get(ctx: Context<Page>) -> Template {
    Template::render("login", &ctx)
}

#[post("/login", data="<data>")]
fn post<'a>(users: State<Users>,
            cookies: &Cookies,
            mut ctx: Context<Page>,
            data: Form<'a, Login>)
            -> Result<Redirect, Template> {
    let mut form = data.into_inner();
    if let Ok(user_name) = form.name.get() {
        match users.get(user_name) {
            Some(ref password) if Ok(password) == form.password.get() => {
                cookies.insert("user_name", user_name);
                cookies.insert("hash",
                               Session::hash(user_name, ctx.session.extra_data, password));
                return Ok(Redirect::to(form.redirect.get().unwrap_or("/login")));
            }
            Some(_) => form.password.set_msg("wrong password".to_string()),
            None => form.name.set_msg("user not found".to_string()),
        }
    }
    ctx.site.login = form;
    Err(Template::render("login", ctx))
}
