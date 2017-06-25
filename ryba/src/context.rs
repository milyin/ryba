use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};
use ryba_kit::form::Field;
use ryba_kit::auth::Session;

#[derive(Serialize,FromForm,Default)]
pub struct Login {
    pub name: Field<String>,
    pub password: Field<String>,
    pub redirect: Field<String>,
}

#[derive(Serialize, Default)]
pub struct Site {
    pub title: String,
    pub login: Login,
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
                    login: Login {
                        redirect: Field::new(request.uri().to_string()),
                        ..Login::default()
                    },
                    ..Site::default()
                },
            }
        )
    }
}


