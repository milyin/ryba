use context::*;
use ryba_kit::template::*;
use pages::*;
use rocket::State;
use std::sync::Mutex;
use Users;

#[derive(Serialize, Default, Debug)]
pub struct Page {
    pub users: Users,
}

#[get("/")]
pub fn get(_users: State<Mutex<Users>>, mut ctx: Context<Page>) -> Template {
    if let Ok(users) = _users.inner().lock() {
        ctx.session.check(&users);
        ctx.page.users = users.clone();
    }
    Template::render("index", &ctx)
}
