use context::*;
use ryba_kit::template::*;
use pages::*;
use rocket::State;
use std::sync::Mutex;
use Users;

#[derive(Serialize, Default)]
pub struct Page {
    pub users: Users
}

#[get("/")]
pub fn get(
    _users: State<Mutex<Users>>,
    mut ctx: Context<Page>) -> Template 
{
    if let Ok(guard) = _users.inner().lock() {
        ctx.page.users = guard.clone();
    }
    Template::render("index", &ctx)
}
