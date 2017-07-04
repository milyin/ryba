use serde::ser::Serialize;
use rocket;
use rocket::request::{self, FromRequest};
use ryba_kit::form::{Field, ContextField};
use ryba_kit::auth::hash;
use std::fmt::Debug;
use Users;

#[derive(FromForm,ToContext)]
pub struct Login<'a> {
    pub name: Field<'a, String>,
    pub password: Field<'a, String>,
    pub redirect: Field<'a, String>,
}

#[derive(Serialize, Default,Debug)]
pub struct Site {
    pub title: String,
    pub login: LoginContext,
    pub layout: &'static str,
}

#[derive(Serialize,Debug)]
pub struct Req {
    pub uri: String,
}

#[derive(Serialize,Debug)]
pub struct Session {
    pub user_name: Option<String>,
    pub client_info: String,
    pub client_hash: Option<u64>,
    pub server_hash: Option<u64>,
    pub logged_in: bool,
}

impl Session {
    pub fn check(&mut self, users: &Users) -> bool {
        self.server_hash = self.user_name
            .clone()
            .and_then(|user_name| {
                          users
                              .get(&user_name)
                              .map(|password| hash(&user_name, &self.client_info, &password))
                      });
        self.logged_in = self.client_hash.is_some() && self.client_hash == self.server_hash;
        println!("{:?}", self.logged_in);
        self.logged_in
    }
}

#[derive(Serialize,Debug)]
pub struct Context<P>
    where P: Serialize + Default
{
    pub req: Req,
    pub session: Session,
    pub page: P,
    pub site: Site,
}

fn get_cookie(request: &rocket::Request, cookie_name: &'static str) -> Option<String> {
    request
        .cookies()
        .find(cookie_name)
        .map(|cookie| cookie.value().to_string())
}

fn get_client_info(request: &rocket::Request) -> String {
    let ip = request
        .remote()
        .map(|addr| addr.ip().to_string())
        .unwrap_or("".to_string());
    let real_ip = request.headers().get_one("X-Real-IP").unwrap_or("");
    return ip + " " + real_ip;
}

impl<'a, 'r, P> FromRequest<'a, 'r> for Context<P>
    where P: Serialize + Default + Debug
{
    type Error = ();
    fn from_request(request: &'a rocket::Request) -> request::Outcome<Self, Self::Error> {
        rocket::Outcome::Success(Context::<P> {
                                     req: Req { uri: request.uri().to_string() },
                                     session: Session {
                                         user_name: get_cookie(request, "user_name"),
                                         client_info: get_client_info(request),
                                         client_hash:
                                             get_cookie(request, "hash").and_then(|v| {
                                                                                      v.parse().ok()
                                                                                  }),
                                         server_hash: None,
                                         logged_in: false,
                                     },
                                     page: P::default(),
                                     site: Site {
                                         layout: "layout",
                                         login: LoginContext {
                                             redirect: ContextField::<String> {
                                                 value: Ok(request.uri().to_string()),
                                                 msg: None,
                                             },
                                             ..LoginContext::default()
                                         },
                                         ..Site::default()
                                     },
                                 })
    }
}
