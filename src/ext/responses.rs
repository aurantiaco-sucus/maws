use maws_http::{AsCaseInsensitive, Status};
use crate::{handler, http};

pub fn ok(req: &http::Request, mime: impl Into<String>, content: impl Into<Vec<u8>>) -> handler::Response {
    let mime = mime.into();
    let content = content.into();
    let mut headers = http::Headers::default();
    headers.insert("Content-Type".case_insensitive(), mime);
    headers.insert("Content-Length".case_insensitive(), content.len().to_string());
    handler::Response {
        http: http::Response {
            version: req.version,
            status: Status::Ok200,
            headers,
        },
        body: Some(content)
    }
}