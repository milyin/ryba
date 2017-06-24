use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};
use ryba_kit::form::Field;

#[derive(Serialize, Default)]
pub struct Form<F>
    where F: Serialize + Default
{
    pub msg: String,
    pub fields: F,
}

#[derive(Serialize,FromForm,Default)]
pub struct Login {
    name: Field<String>,
    password: Field<String>,
}

#[derive(Serialize, Default)]
pub struct Site {
    pub title: String,
    pub login: Form<Login>,
    pub layout: &'static str,
}

#[derive(Serialize)]
pub struct Req {
    pub uri: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Req {
    type Error = ();
    fn from_request(request: &'a rocket::Request) -> request::Outcome<Self, Self::Error> {
        rocket::Outcome::Success(Req { uri: request.uri().to_string() })
    }
}

#[derive(Serialize)]
pub struct Context<P>
    where P: Serialize + Default
{
    pub req: Req,
    pub page: P,
    pub site: Site,
}

impl<P> Context<P>
    where P: Serialize + Default
{
    pub fn new(req: Req, page: P) -> Context<P> {
        Context::<P> {
            req: req,
            page: page,
            site: Site {
                layout: "layout",
                ..Site::default()
            },
        }
    }
}
