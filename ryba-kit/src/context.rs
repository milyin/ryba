use rocket;
use rocket::request::{self, FromRequest};
use serde::ser::Serialize;

#[derive(Serialize)]
pub struct Request {
    uri: String
}

#[derive(Serialize)]
pub struct Context<'f, F> where F: Serialize + 'f {
    req: Request,
    form: Option<&'f F>
}

impl <'a, 'r> FromRequest<'a,'r> for Request
{
    type Error = ();
    fn from_request(request: &'a rocket::Request) -> request::Outcome<Self, Self::Error>
    {
        rocket::Outcome::Success(Request {
            uri: request.uri().to_string()
        })
    }
}

pub fn simple(req: Request) -> Context<'static,()> {
    Context {
        req: req,
        form: None
    }
}

pub fn form<'f,F>(req: Request, form: &'f F) -> Context<'f,F> where F: Serialize + 'f {
    Context {
        req: req,
        form: Some(form)
    }
}
