use std::fmt::{Debug, Formatter};
use crate::http;

pub struct Handler(Box<dyn Fn(http::Request) -> (http::Response, Option<Vec<u8>>) + Send + Sync>);

impl Debug for Handler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Handler@{:?}>", self.0.as_ref() as *const _)
    }
}

impl Handler {
    fn handle(&self, req: http::Request) ->  (http::Response, Option<Vec<u8>>) {
        self.0(req)
    }
}

pub fn primitive(func: impl Fn(http::Request) ->  (http::Response, Option<Vec<u8>>) + Send + Sync + 'static) -> Handler {
    Handler(Box::new(func))
}

pub fn not_found_404() -> Handler {
    primitive(|req| {
        (http::Response {
            version: req.version,
            status: http::Status::NotFound404,
            headers: http::Headers::default(),
        }, None)
    })
}