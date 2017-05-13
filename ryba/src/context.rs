use ryba_kit::context;
use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};

#[derive(Serialize)]
pub struct Site {
    pub title: &'static str
}

#[derive(Serialize)]
pub struct Page<F> where F: Serialize {
    pub form: Option<F>,
    pub title: String
}

#[derive(Serialize)]
pub struct Context<F> where F: Serialize {
    pub req: Option<context::Request>,
    pub site: Site,
    pub page: Page<F>
}

impl <'a, 'r, F> FromRequest<'a,'r> for Context<F> where F: Serialize
{
    type Error = ();
    fn from_request(request: &'a rocket::Request) -> request::Outcome<Self, Self::Error>
    {
        rocket::Outcome::Success(Context {
            req: context::Request::from_request(request).succeeded(),
            site : Site {
                title: ""
            },
            page: Page {
                form: None,
                title: "".to_string()
            }
        })
    }
}