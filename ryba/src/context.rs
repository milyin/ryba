use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};

#[derive(Serialize, Default)]
pub struct Site {
    pub title: String,
}

#[derive(Serialize, Default)]
pub struct Page {
    pub title: String,
}

#[derive(Serialize)]
pub struct Req {
    pub uri: String,
}

impl <'a, 'r> FromRequest<'a,'r> for Req
{
    type Error = ();
    fn from_request(request: &'a rocket::Request) -> request::Outcome<Self, Self::Error>
    {
        rocket::Outcome::Success( Req {
            uri: request.uri().to_string()
        })
    }
}

#[derive(Serialize)]
pub struct Context<'a, F> where F : 'a + Serialize {
    pub req: Req,
    pub site: Site,
    pub page: Page,
    pub layout: &'static str,
    pub form: &'a F
}

impl <'a,F> Context<'a,F> where F : 'a + Serialize {
  pub fn new(req: Req, form: &'a F) -> Context<'a,F> {
      Context::<F> {
          req: req,
          form: form,
          site: Site::default(),
          page: Page::default(),
          layout: "layout"
      }
  }
}
