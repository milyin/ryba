use context::*;
use ryba_kit::template::*;
use pages::*;

#[get("/")]
pub fn get(req: Req) -> Template {
    let ctx = Context::new(req, ());
    Template::render("index", &ctx)
}

login_handler!(login_post, "/?login", "index", ());
