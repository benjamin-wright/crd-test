use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::{Header, Method};

pub struct CORS {}

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS header insert",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Get || request.method() == Method::Post {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }
    }
}