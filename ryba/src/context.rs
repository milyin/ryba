use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};
use ryba_kit::form::{Field,ContextField};
use ryba_kit::auth::Session;

#[derive(FromForm,ToContext)]
pub struct Login<'a> {
    pub name: Field<'a, String>,
    pub password: Field<'a, String>,
    pub redirect: Field<'a, String>,
}

#[derive(Serialize, Default)]
pub struct Site {
    pub title: String,
    pub login: LoginContext,
    pub layout: &'static str,
}

#[derive(Serialize)]
pub struct Req {
    pub uri: String,
}

#[derive(Serialize)]
pub struct Context<P>
    where P: Serialize + Default
{
    pub req: Req,
    pub session: Session,
    pub page: P,
    pub site: Site,
}

impl<'a, 'r, P> FromRequest<'a, 'r> for Context<P> 
    where P: Serialize + Default
{
    type Error = ();
    fn from_request(request: &'a rocket::Request) -> request::Outcome<Self, Self::Error> {
        rocket::Outcome::Success(
            Context::<P> {
                req: Req { 
                    uri: request.uri().to_string() 
                },
                session: request.into(),
                page: P::default(),
                site: Site {
                    layout: "layout",
                    login: LoginContext {
                        redirect: ContextField::<String> {
                            value: Ok(request.uri().to_string()),
                            msg: None
                        },
                        ..LoginContext::default()
                    },
                    ..Site::default()
                },
            }
        )
    }
}


