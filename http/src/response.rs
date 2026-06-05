use crate::{Headers, Status, Version};

pub struct Response {
    pub version: Version,
    pub status: Status,
    pub headers: Headers,
}

impl From<&Response> for Vec<u8> {
    fn from(
        Response {
            version,
            status,
            headers,
        }: &Response,
    ) -> Self {
        let mut buf = Vec::with_capacity(512);
        buf.extend(version.as_str().bytes());
        buf.push(b' ');
        buf.extend(status.as_str().bytes());
        buf.extend(b"\r\n");
        for (k, v) in headers.iter() {
            for v in v {
                buf.extend(k.case_sensitive_ref().trim_ascii().bytes());
                buf.extend(b": ");
                buf.extend(v.bytes());
                buf.extend(b"\r\n");
            }
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    // quote from https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Messages
    const RESPONSE: &[u8] = b"\
    HTTP/1.1 201 Created\r\n\
    Content-Type: application/json\r\n\
    Location: http://example.com/users/123\r\n\
    \r\n\
    {\n\
      \"message\": \"New user created\",\n\
      \"user\": {\n\
        \"id\": 123,\n\
        \"firstName\": \"Example\",\n\
        \"lastName\": \"Person\",\n\
        \"email\": \"bsmth@example.com\"\n\
      }\n\
    }\n\
    ";

    #[test]
    fn test_parse() {
        // todo
    }
}