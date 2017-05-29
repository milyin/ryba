use ryba_kit::context;
use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};
use std::borrow::Cow;
use ryba_kit::template::*;

macro_rules! cow_setter { ($field:ident, $setter:ident) => (
    pub fn $setter<S>(&mut self, s: S) where S: Into<Cow<'static,str>> {
        self.$field = s.into();
    }
)}

#[derive(Serialize, Default)]
pub struct Site {
    pub title: Cow<'static, str>
}

impl Site {
    cow_setter!(title, set_title);
}

#[derive(Serialize, Default)]
pub struct Page {
    pub layout: Cow<'static,str>,
    pub title: Cow<'static,str>,
}

impl Page {
    cow_setter!(layout, set_layout);
    cow_setter!(title, set_title);
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
