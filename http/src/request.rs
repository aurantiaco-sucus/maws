use crate::{AsCaseInsensitive, HeaderPolicy, Headers, Method, Version};
use anyhow::Context;

pub struct Request {
    pub method: Method,
    pub target: RequestTarget,
    pub version: Version,
    pub headers: Headers,
}

#[derive(Debug, Eq, PartialEq)]
pub enum RequestTarget {
    Origin(String),
    Absolute(String),
    Authority(String, u16),
}

impl RequestTarget {
    pub fn parse(src: &str) -> anyhow::Result<Self> {
        if src.starts_with('/') {
            return Ok(RequestTarget::Origin(src.to_owned()))
        }
        if src.contains(':') && !src.contains('/') {
            let (authority, port) = src.split_once(':').unwrap();
            let port = str::parse(port).context("unable to parse port number to u16")?;
            return Ok(RequestTarget::Authority(authority.to_string(), port))
        }
        Ok(RequestTarget::Absolute(src.to_owned()))
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

impl Request {
    pub fn parse(value: &str, policy: &RequestPolicy) -> anyhow::Result<Self> {
        let start = value.lines().next().unwrap();
        let value = &value[start.len()..];
        let (method, path, version) = parse_start(start)?;
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
        let values = if let Some(values) = self.headers.get("content-length".case_insensitive()) {
            values
        } else {
            return Ok(None);
        };
        if values.len() == 1 {
            let value = values[0].parse::<u64>().with_context(|| {
                format!("unable to convert Content-Length value \"{}\" to u64", values[0])
            })?;
            Ok(Some(value))
        } else {
            match policy.duplicate_singleton {
                RequestDuplicatSingletonPolicy::Reject => {
                    anyhow::bail!("Content-Length is duplicated in request headers")
                }
                RequestDuplicatSingletonPolicy::First => {
                    let value = values[0].parse::<u64>().with_context(|| {
                        format!("unable to convert Content-Length value \"{}\" to u64", values[0])
                    })?;
                    Ok(Some(value))
                }
                RequestDuplicatSingletonPolicy::Last => {
                    let value = values.last().unwrap().parse::<u64>().with_context(|| {
                        format!(
                            "unable to convert Content-Length value \"{}\" to u64",
                            values.last().unwrap()
                        )
                    })?;
                    Ok(Some(value))
                }
            }
        }
    }
}

fn parse_start(line: &str) -> anyhow::Result<(Method, &str, Version)> {
    let parts = line.split(' ').collect::<Vec<_>>();
    if parts.len() != 3 {
        anyhow::bail!("request line is malformed: {line}");
    }
    let method = Method::parse(parts[0].case_insensitive())
        .with_context(|| format!("HTTP method is unknown: {}", parts[0]))?;
    let version = Version::parse(parts[1].case_insensitive())
        .with_context(|| format!("HTTP version is unknown: {}", parts[0]))?;
    Ok((method, parts[1], version))
}

#[cfg(test)]
mod tests {
    use super::*;

    // quote from https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Messages
    const REQUEST: &str = "\
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
            target,
            version,
            headers,
        } = req;
        assert_eq!(method, Method::POST);
        assert_eq!(target, RequestTarget::Origin("/users".to_string()));
        assert_eq!(version, Version::Http11);
        assert_eq!(
            headers.get("content-type".case_insensitive()).unwrap(),
            &["application/x-www-form-urlencoded".to_string()]
        );
    }
}
