use crate::util::ByteSliceRefExt;
use crate::{ByteStr, ByteString, HeaderPolicy, Headers, Method, ToByteStr, Version, byte_str};
use anyhow::Context;

pub struct Request {
    pub method: Method,
    pub path: ByteString<true>,
    pub version: Version,
    pub headers: Headers,
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
        let path = path.to_owned();
        let headers = Headers::parse(&value, &policy.header)?;
        Ok(Self {
            method,
            path,
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
            path,
            version,
            headers,
        } = req;
        assert_eq!(method, Method::POST);
        assert_eq!(path.as_ref(), byte_str!(s:b"/users"));
        assert_eq!(version, Version::Http11);
        assert_eq!(headers.get(b"content-type").unwrap(), [byte_str!(s:b"application/x-www-form-urlencoded")]);
    }
}
