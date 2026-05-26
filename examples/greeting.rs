use std::collections::{BTreeMap, HashMap};
use std::net::SocketAddr;
use std::str::FromStr;
use maws::http;

http::const_byte_str!(GREETING = s:b"Hello, World!");
http::const_byte_str!(GREETING_LEN = s:b"13");

fn main() {
    let config = maws::Config {
        endpoints: HashMap::from([
            (http::byte_str!(s:b"/").to_owned(), BTreeMap::from([
                maws::endpoint(http::Method::GET, |req: maws::Request| {
                    let mut headers = http::Headers::default();
                    headers.insert(b"Content-Length", GREETING_LEN.to_owned());
                    maws::Response {
                        http: http::Response {
                            version: req.http.version,
                            status: http::Status::Ok200,
                            headers,
                        },
                        body: Some(GREETING.bytes().to_owned())
                    }
                })
            ]))
        ]),
        default_endpoint: maws::endpoint_func(|req| {
            maws::Response {
                http: http::Response {
                    version: req.http.version,
                    status: http::Status::NotFound404,
                    headers: http::Headers::default(),
                },
                body: None
            }
        }),
        addr: SocketAddr::from_str("127.0.0.1:60080").unwrap(),
        handler_config: maws::HandlerConfig::default(),
    };
    maws::ignite(config).unwrap();
}