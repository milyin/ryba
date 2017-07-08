use context::*;
use ryba_kit::template::*;
use ryba_kit::auth::*;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use users::*;

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
fn post_login<'a>(cookies: &Cookies,
                  mut ctx: Context<Page>,
                  data: Form<'a, Login<'a>>)
                  -> Result<Redirect, Template> {
    let form = data.get();
    ctx.site.login = form.context();
    if let Some((name, password, redirect)) = form.values() {
        match check_user(name, password) {
            Ok(_) => {
                cookies.add(Cookie::new("user_name", name.clone()));
                cookies.add(Cookie::new("hash",
                                        hash(name, &ctx.session.client_info, password)
                                            .to_string()));
                return Ok(Redirect::to(redirect));
            }
            Err(msg) => {
                ctx.site.login.name.msg = Some(msg);
            }
        }
    }
    Err(Template::render("login", ctx))
}

#[post("/logout", data="<data>")]
fn post_logout<'a>(cookies: &Cookies, data: Form<'a, Login<'a>>) -> Redirect {
    cookies.remove("user_name");
    cookies.remove("hash");
    if let Some((_, _, redirect)) = data.get().values() {
        Redirect::to(redirect)
    } else {
        Redirect::to("/")
    }
}
