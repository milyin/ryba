use context::*;
use ryba_kit::template::*;

#[derive(Serialize, Default, Debug)]
pub struct Page {}

#[get("/")]
pub fn get(ctx: Context<Page>) -> Template {
    Template::render("index", &ctx)
}
