use context::*;
use ryba_kit::template::*;
use ryba_kit::form::*;
use ryba_kit::auth::*;
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket::State;
use pages::*;
use Users;
use std::sync::Mutex;

#[derive(Serialize,Debug)]
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
fn post<'a>(_users: State<Mutex<Users>>,
            cookies: &Cookies,
            mut ctx: Context<Page>,
            data: Form<'a, Login<'a>>)
            -> Result<Redirect, Template> {
    if let Ok(users) = _users.inner().lock() {
        let form = data.get();
        ctx.site.login = form.context();
        if let Some((name, password, redirect)) = form.values() {
            match users.get(name) {
                Some(correct_password) if password == correct_password => {
                    cookies.add(Cookie::new("user_name", name.clone()));
                    cookies.add(Cookie::new("hash",
                                            hash(name, &ctx.session.client_info, password)
                                                .to_string()));
                    return Ok(Redirect::to(redirect));
                }
                Some(_) => ctx.site.login.password.msg = Some("wrong password".to_string()),
                None => ctx.site.login.name.msg = Some("User not found".to_string()),
            }

        }
    }
    Err(Template::render("login", ctx))
}
