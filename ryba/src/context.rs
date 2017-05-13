use ryba_kit::context;
use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};
use rocket_contrib::Template;

#[derive(Serialize, Default)]
pub struct Site {
    pub title: &'static str
}

#[derive(Serialize, Default)]
pub struct Page {
    pub title: String
}

#[derive(Serialize, Default)]
pub struct Context {
    pub req: Option<context::Request>,
    pub site: Site,
    pub page: Page
}

impl <'a, 'r> FromRequest<'a,'r> for Context
{
    type Error = ();
    fn from_request(request: &'a rocket::Request) -> request::Outcome<Self, Self::Error>
    {
        let mut ctx = Context::default();
        ctx.req = context::Request::from_request(request).succeeded();
        rocket::Outcome::Success(ctx)
    }
}

#[derive(Serialize)]
pub struct Root<'a, F> where F : 'a + Serialize {
    req: &'a Option<context::Request>,
    site: &'a Site,
    page: &'a Page,
    form: &'a F
}

pub fn render_with_form<F>(name: &'static str, ctx: &Context, form: &F) -> Template where F: Serialize {
    Template::render(name, &Root::<F> {
        req: &ctx.req,
        site: &ctx.site,
        page: &ctx.page,
        form: form
    } )
}

pub fn render(name: &'static str, ctx: &Context) -> Template {
    render_with_form(name,ctx,&())
}
