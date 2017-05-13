#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#[macro_use]

extern crate ryba_kit;
extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod context;

use ryba_kit::form::*;
use serde::ser::Serialize;
use rocket_contrib::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use context::Context;

// TODO: allow only own urls
#[derive(FromForm)]
struct OwnUrl {
    url: String
}

ryba_form! { form: Login () ctx: LoginCtx  {
    Input, text, name: String, pass, String,
    Input, password, password: String, pass, String
} () }

#[get("/login?<backurl>")]
fn login(backurl: OwnUrl, req: ryba_kit::context::Request) -> Template {
    let _unused = backurl;
    let login = Login::default();
    let login_ctx = LoginCtx::validate(&login);
    Template::render("login", &ryba_kit::context::form(req,&login_ctx))
}

#[post("/login?<backurl>", data="<login_form>")]
fn login_post(backurl: OwnUrl, login_form: Form<Login>, req: ryba_kit::context::Request) -> Result<Redirect,Template> {
    let login = login_form.get();
    let login_ctx = LoginCtx::validate(login);
    if login_ctx.is_ok() &&
        login.name == "foo".to_string() && login.password == "bar".to_string() {
        Ok(Redirect::to(&backurl.url))
    } else {
        Err(Template::render("login",&ryba_kit::context::form(req,&login_ctx)))
    }
}

ryba_form! { 
    form:Register (name: &'static str) ctx:RegisterCtx  {
        Input, text, name: String, |x| { if x==name {pass(x)} else {fail("Not name".to_string())} }, String,
        Input, password, password: String, pass, String,
        Input, password, password1: String, pass /*|x| {if x==&form.password {pass(x)} else {fail("Password not match".to_string())}}*/, String
    } {
      if ctx.password.is_ok() && ctx.password1.is_ok() && 
        ctx.password.get().unwrap() != ctx.password1.get().unwrap() {
            ctx.password.set_msg("NOT MATCH".to_string());
        }
    }
}

#[get("/register")]
fn register(req: ryba_kit::context::Request) -> Template {
    let frm = Register::default();
    let ctx = RegisterCtx::validate(&frm,"foo");
    Template::render("register", &ryba_kit::context::form(req,&ctx))
}

#[post("/register", data="<form>")]
fn register_post(form: Form<Register>, req: ryba_kit::context::Request) -> Result<Redirect,Template> {
    let frm = form.get();
    let ctx = RegisterCtx::validate(frm,"foo");
    if ctx.is_ok() {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("register",&ryba_kit::context::form(req,&ctx)))
    }
}

#[get("/")]
fn index(mut ctx : Context<()>) -> Template {
    ctx.page.title = "root page".to_string();
    Template::render("index", &ctx)
}

fn main() {
    rocket::ignite().mount("/", routes![index, login, login_post, register, register_post]).launch();
}
