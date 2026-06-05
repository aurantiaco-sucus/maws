use maws::ext::responses;
use maws::{handler, http, route};
use std::net::SocketAddr;
use std::str::FromStr;

http::const_byte_str!(GREETING = s:b"Hello, World!");
http::const_byte_str!(GREETING_LEN = s:b"13");

fn main() {
    let mut routes = route::Routes::empty();

    routes.insert(route::pattern("/"), http::Method::GET, handler::primitive(|req| {
        responses::ok(&req.http, "text/plain", "Hello, World!")
    }));

    routes.insert(route::pattern("/**"), http::Method::GET, handler::standard_404());

    let config = maws::Config {
        routes,
        addr: SocketAddr::from_str("127.0.0.1:60080").unwrap(),
        handler_config: maws::HandlerConfig::default(),
    };
    maws::ignite(config).unwrap();
}