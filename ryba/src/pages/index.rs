use context::*;
use ryba_kit::template::*;
use pages::*;

#[get("/")]
pub fn get(ctx: Context<()>) -> Template {
    Template::render("index", &ctx)
}
