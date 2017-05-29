#![feature(plugin, custom_derive, closure_to_fn_coercion)]
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
use rocket::request::Form;
use rocket::response::Redirect;
use context::*;
use ryba_kit::template::*;

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
fn login(ctx: Context, backurl: OwnUrl) -> Template {
    let _unused = backurl;
    let form = LoginCtx::validate(&Login::default());
    render_with_form("login", &ctx, &form)
}

#[post("/login?<backurl>", data="<data>")]
fn login_post(ctx: Context, backurl: OwnUrl, data: Form<Login>) -> Result<Redirect,Template> {
    let form = LoginCtx::validate(data.get());
    if form.is_ok() {
        Ok(Redirect::to(&backurl.url))
    } else {
        Err(render_with_form("login", &ctx, &form))
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
fn register(ctx: Context) -> Template {
    let form = RegisterCtx::validate(&Register::default(),"foo");
    render_with_form("register", &ctx, &form)
}

#[post("/register", data="<data>")]
fn register_post(ctx: Context, data: Form<Register>) -> Result<Redirect,Template> {
    let form = RegisterCtx::validate(data.get(),"foo");
    if form.is_ok() {
        Ok(Redirect::to("/"))
    } else {
        Err(render_with_form("register", &ctx, &form))
    }
}

#[get("/")]
fn index(mut ctx : Context) -> Template {
    ctx.page.set_title("root page");
    render("index", &ctx)
}

#[get("/hbs")]
fn hbs(mut ctx : Context) -> Template {
    ctx.page.set_layout("layout");
    ctx.page.set_title("root page".to_string());
    render("index", &ctx)
}


fn main() {
    init_handlebars(|hb| {});
    add_templates("templates").expect("Failed to read templates");
    rocket::ignite().mount("/", routes![index, login, login_post, register, register_post, hbs]).launch();
}
