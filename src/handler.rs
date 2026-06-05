use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::http;

pub struct Request {
    http: http::Request,
    arguments: Vec<String>,
    query: HashMap<String, String>,
}

pub struct Response {
    http: http::Response,
    body: Option<Vec<u8>>
}

pub struct Handler(Box<dyn Fn(Request) -> Response + Send + Sync>);

impl Debug for Handler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Handler@{:?}>", self.0.as_ref() as *const _)
    }
}

impl Handler {
    fn handle(&self, req: Request) -> Response {
        self.0(req)
    }
}

pub fn primitive(func: impl Fn(Request) -> Response + Send + Sync + 'static) -> Handler {
    Handler(Box::new(func))
}

pub fn standard_404() -> Handler {
    primitive(|req| {
        Response {
            http: http::Response {
                version: req.http.version,
                status: http::Status::NotFound404,
                headers: http::Headers::default(),
            },
            body: Some("404 Not Found".as_bytes().to_vec())
        }
    })
}