use crate::util::ByteSliceRefExt;
use crate::{ByteStr, ByteString, HeaderPolicy, Headers, Method, ToByteStr, Version, byte_str};
use anyhow::Context;

pub struct Request {
    pub method: Method,
    pub target: RequestTarget,
    pub version: Version,
    pub headers: Headers,
}

pub enum RequestTarget {
    Origin(Vec<String>),
    Absolute(String),
    Authority(String, u16),
}

impl RequestTarget {
    pub fn parse(src: &ByteStr<true>) -> anyhow::Result<Self> {
        if src.bytes()[0] == b'/' {
            if src.bytes().len() == 1 {
                return Ok(RequestTarget::Origin(Vec::new()));
            }
            let mut segments = Vec::new();
            for segment in src.bytes()[1..]
                .case_sensitive()
                .split_pat_naive(byte_str!(s:b"/"))
            {
                segments.push(
                    segment
                        .as_str()
                        .context("unable to parse request target into UTF-8")?
                        .to_owned(),
                );
            }
            return Ok(RequestTarget::Origin(segments))
        }
        if src.bytes().contains(&b':') && !src.bytes().contains(&b'/') {
            let i_port = src.bytes().iter().position(|x| x == &b':').unwrap();
            let authority = src.bytes()[..i_port].case_sensitive().as_str()
                .context("authority is not valid UTF-8")?
                .to_owned();
            let i_port = src.bytes()[i_port..].case_insensitive().as_str()
                .context("port number contains invalid character(s)")?;
            let port = str::parse(i_port)
                .context("unable to parse port number to u16")?;
            return Ok(RequestTarget::Authority(authority, port))
        }
        let absolute = src.as_str()
            .context("unable to parse the absolute form target to string")?
            .to_owned();
        Ok(RequestTarget::Absolute(absolute))
    }
}

#[derive(Clone, Default)]
pub struct RequestPolicy {
    pub duplicate_singleton: RequestDuplicatSingletonPolicy,
    pub header: HeaderPolicy,
}

#[derive(Copy, Clone)]
pub enum RequestDuplicatSingletonPolicy {
    /// reject the request
    Reject,
    /// use the first value
    First,
    /// use the last value
    Last,
}

#[allow(clippy::derivable_impls)]
impl Default for RequestDuplicatSingletonPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(value, &Default::default())
    }
}

impl Request {
    pub fn parse(mut value: &[u8], policy: &RequestPolicy) -> anyhow::Result<Self> {
        let start = value
            .take_until_dropped_pattern(b"\r\n")
            .context("request is malformed")?;
        let (method, path, version) = parse_start(start.case_insensitive())?;
        let target = RequestTarget::parse(path)?;
        let headers = Headers::parse(&value, &policy.header)?;
        Ok(Self {
            method,
            target,
            version,
            headers,
        })
    }

    pub fn content_length(&self, policy: &RequestPolicy) -> anyhow::Result<Option<u64>> {
        let values = if let Some(values) = self.headers.get(b"content-length") {
            values
        } else {
            return Ok(None);
        };
        if values.len() == 1 {
            let value = values[0]
                .as_str()
                .context("Content-Length is not valid UTF-8")?;
            let value = value.parse::<u64>().with_context(|| {
                format!(
                    "unable to convert Content-Length value \"{}\" to u64",
                    value
                )
            })?;
            Ok(Some(value))
        } else {
            match policy.duplicate_singleton {
                RequestDuplicatSingletonPolicy::Reject => {
                    anyhow::bail!("Content-Length is duplicated in request headers")
                }
                RequestDuplicatSingletonPolicy::First => {
                    let value = values
                        .first()
                        .unwrap()
                        .as_str()
                        .context("Content-Length is not valid UTF-8")?;
                    let value = value.parse::<u64>().with_context(|| {
                        format!(
                            "unable to convert Content-Length value \"{}\" to u64",
                            value
                        )
                    })?;
                    Ok(Some(value))
                }
                RequestDuplicatSingletonPolicy::Last => {
                    let value = values
                        .last()
                        .unwrap()
                        .as_str()
                        .context("Content-Length is not valid UTF-8")?;
                    let value = value.parse::<u64>().with_context(|| {
                        format!(
                            "unable to convert Content-Length value \"{}\" to u64",
                            value
                        )
                    })?;
                    Ok(Some(value))
                }
            }
        }
    }
}

fn parse_start(line: &ByteStr<false>) -> anyhow::Result<(Method, &ByteStr<true>, Version)> {
    let mut parts = line.split_pat_naive(byte_str!(i:b" "));
    let method = parts.next().unwrap().try_into()?;
    let path = parts.next().context("missing path")?.case_sensitive();
    let version = parts.next().context("missing version")?.try_into()?;
    Ok((method, path, version))
}

#[cfg(test)]
mod tests {
    use super::*;

    // quote from https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Messages
    const REQUEST: &[u8] = b"\
    POST /users HTTP/1.1\r\n\
    Host: example.com\r\n\
    Content-Type: application/x-www-form-urlencoded\r\n\
    Content-Length: 49\r\n\
    ";

    #[test]
    fn test_parse() {
        let pol = RequestPolicy::default();
        let req = Request::parse(REQUEST, &pol).unwrap();
        assert_eq!(req.content_length(&pol).unwrap().unwrap(), 49);
        let Request {
            method,
            target: path,
            version,
            headers,
        } = req;
        assert_eq!(method, Method::POST);
        assert_eq!(path.as_ref(), byte_str!(s:b"/users"));
        assert_eq!(version, Version::Http11);
        assert_eq!(
            headers.get(b"content-type").unwrap(),
            [byte_str!(s:b"application/x-www-form-urlencoded")]
        );
    }
}
