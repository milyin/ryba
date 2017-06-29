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
fn post<'a>(users: State<Mutex<Users>>,
            cookies: &Cookies,
            mut ctx: Context<Page>,
            data: Form<'a, Login>)
            -> Result<Redirect, Template> 
{
/*    let mut form = data.into_inner();
    if let (&Ok(ref user_name), &Ok(ref password), &Ok(ref redirect)) =
        (&form.name.value, &form.password.value, &form.redirect.value) {
        match users.get(user_name) {
            Some(correct_password) if password == correct_password => {
                cookies.add(Cookie::new("user_name", user_name.clone()));
                cookies.add(Cookie::new("hash",
                                        Session::hash(user_name,
                                                      &ctx.session.extra_data,
                                                      password)
                                                .to_string()));
                return Ok(Redirect::to(redirect));
            }
            Some(_) => form.password.msg = Some("wrong password".to_string()),
            None => form.name.msg = Some("user not found".to_string()),
        }
    }*/
   // ctx.site.login = form;
    Err(Template::render("login", ctx))
}
