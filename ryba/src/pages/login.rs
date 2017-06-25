use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use pages::*;

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
fn post<'a>(cookies: Cookies, mut ctx: Context<Page>, data: Form<'a, Login>) -> Result<Redirect, Template> {
    let mut form = data.into_inner();

    let test_name: String = "foo".to_string();
    let test_password: String = "bar".to_string();

    if form.name.is_ok() && form.password.is_ok() {
        if form.name.get() != Ok(&test_name) {
            form.name.set_msg("user not found".to_string());
        } else if form.password.get() != Ok(&test_password) {
            form.password.set_msg("password not match".to_string());
        } else  {
            let url = form.redirect.get().ok().map_or("/", |s| &s);
            return Ok(Redirect::to(url));
        }
    }

    ctx.site.login = form;
    Err(Template::render("login", ctx))
}
