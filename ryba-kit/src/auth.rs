use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rocket;
use rocket::request::{self, Request, FromRequest};

#[derive(PartialEq, Serialize)]
pub struct Session {
    pub user_name: Option<String>,
    pub extra_data: String,
    pub hash: Option<u64>
}

#[derive(Hash)]
struct SessionData<'a> {
    user_name: &'a str, 
    extra_data: &'a str,
    password: &'a str, 
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Session {
    pub fn hash(user_name: &str, extra_data: &str, password: &str) -> u64 {
        calculate_hash(&SessionData {
                user_name: user_name,
                extra_data: extra_data,
                password: password,
        })
    }
    pub fn new(user_name: String, extra_data: String, password: &str) -> Session {
        let hash = Session::hash(&user_name, &extra_data, password);
        Session {
            user_name: Some(user_name),
            extra_data: extra_data,
            hash: Some(hash)
        }
    }
    pub fn check(&self, user_name: &str, extra_data: &str, password: &str) -> bool {
         self.hash == Some(Session::hash(user_name, extra_data, password))
    }
}

impl<'a, 'r> From<&'a Request<'r>> for Session {
    fn from(request: &'a Request<'r>) -> Session {
        let user_name = request.cookies().find("user_name").map( |cookie| { cookie.value().to_owned() } );
        let hash = match request.cookies().find("hash").map( |cookie| { cookie.value().parse() } ) {
            Some(Ok(v)) => Some(v),
            _ => None
        };
        let ip = request.remote().map(|addr| addr.ip().to_string()).unwrap_or("".to_string());
        let real_ip = request.headers().get_one("X-Real-IP").unwrap_or("");
        Session { 
            user_name: user_name,
            extra_data: ip+" "+real_ip,
            hash: hash,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = !;
    fn from_request(request: &'a Request) -> request::Outcome<Self, Self::Error> {
        rocket::Outcome::Success(Session::from(request))
    }
}