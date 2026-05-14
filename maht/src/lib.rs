use std::collections::HashMap;
use anyhow::Context;

pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: Version,
    pub headers: HashMap<String, Vec<String>>,
}

impl TryFrom<&[u8]> for Request {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(value, &Default::default())
    }
}

impl Request {
    pub fn parse(value: &[u8], policy: &RequestPolicy) -> anyhow::Result<Self> {
        let header = String::from_utf8_lossy(&value);
        let mut lines = header.lines();
        let (method, path, version) = parse_start(lines.next().unwrap())?;
        let path = path.to_string();
        let mut headers: HashMap<String, Vec<String>> = HashMap::new();
        for line in lines {
            let (mut key, value) = line.split_once(':')
                .context("invalid header syntax")?;
            if key.starts_with(' ') || key.ends_with(' ') {
                if policy.allow_untrimmed_header_keys {
                    key = key.trim();
                } else {
                    anyhow::bail!("whitespace is found around request header key")
                }
            }
            let value = value.trim();
            headers.entry(key.to_lowercase().to_string())
                .or_default()
                .push(value.to_string());
        }
        Ok(Self { method, path, version, headers })
    }

    pub fn content_length(&self, policy: &RequestPolicy) -> anyhow::Result<u64> {
        let values = self.headers.get("content-length")
            .context("Content-Length not found in headers")?;
        if values.len() == 1 {
            let value = values[0].parse::<u64>()
                .with_context(|| format!("unable to convert Content-Length value \"{}\" to u64", values[0]))?;
            Ok(value)
        } else {
            match policy.duplicate_singleton {
                RequestDuplicatSingletonPolicy::Reject => {
                    anyhow::bail!("Content-Length is duplicated in request headers")
                }
                RequestDuplicatSingletonPolicy::First => {
                    let value = values.first().unwrap();
                    let value = value.parse::<u64>()
                        .with_context(|| format!("unable to convert Content-Length value \"{}\" to u64", value))?;
                    Ok(value)
                }
                RequestDuplicatSingletonPolicy::Last => {
                    let value = values.last().unwrap();
                    let value = value.parse::<u64>()
                        .with_context(|| format!("unable to convert Content-Length value \"{}\" to u64", value))?;
                    Ok(value)
                }
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct RequestPolicy {
    pub allow_untrimmed_header_keys: bool,
    pub duplicate_singleton: RequestDuplicatSingletonPolicy,
}

#[derive(Copy, Clone)]
pub enum RequestDuplicatSingletonPolicy {
    /// reject the request
    Reject,
    /// use the first value
    First,
    /// use the last value
    Last
}

impl Default for RequestDuplicatSingletonPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

fn parse_start(line: &str) -> anyhow::Result<(Method, &str, Version)> {
    let mut parts = line.split(' ');
    let method = parts.next().unwrap().try_into()?;
    let path = parts.next()
        .context("missing path")?;
    let version = if let Some(version) = parts.next() {
        version.try_into()?
    } else {
        Version::Http09
    };
    Ok((method, path, version))
}

pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl TryFrom<&str> for Method {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(Method::GET),
            "head" => Ok(Method::HEAD),
            "post" => Ok(Method::POST),
            "put" => Ok(Method::PUT),
            "delete" => Ok(Method::DELETE),
            "connect" => Ok(Method::CONNECT),
            "options" => Ok(Method::OPTIONS),
            "trace" => Ok(Method::TRACE),
            "patch" => Ok(Method::PATCH),
            _ => Err(anyhow::anyhow!("unknown method: {value}"))
        }
    }
}

impl From<Method> for &'static str {
    fn from(value: Method) -> Self {
        match value {
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::CONNECT => "CONNECT",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
        }
    }
}

pub enum Version {
    Http09,
    Http10,
    Http11,
    Http20,
}

impl TryFrom<&str> for Version {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "http/0.9" => Ok(Version::Http09),
            "http/1.0" => Ok(Version::Http10),
            "http/1.1" => Ok(Version::Http11),
            "http/2.0" => Ok(Version::Http20),
            _ => Err(anyhow::anyhow!("unknown version: {value}"))
        }
    }
}

impl From<Version> for &'static str {
    fn from(value: Version) -> Self {
        match value {
            Version::Http09 => "HTTP/0.9",
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
            Version::Http20 => "HTTP/2.0",
        }
    }
}

pub struct Response {

}