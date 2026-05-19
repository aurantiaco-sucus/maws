mod status;
mod headers;
mod util;
mod byte_string;
mod method;
mod version;

use anyhow::{Context};
use std::collections::HashMap;

pub use status::*;
pub use headers::*;
pub use byte_string::*;
pub use method::*;
pub use version::*;
use crate::util::ByteSliceRefExt;

pub struct Request {
    pub method: Method,
    pub path: ByteString<true>,
    pub version: Version,
    pub headers: Headers,
}

impl TryFrom<&[u8]> for Request {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(value, &Default::default())
    }
}

impl Request {
    pub fn parse(mut value: &[u8], policy: &RequestPolicy) -> anyhow::Result<Self> {
        let start = value.take_until_dropped_pattern(b"\r\n")
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
            let value = values[0].parse::<u64>().with_context(|| {
                format!(
                    "unable to convert Content-Length value \"{}\" to u64",
                    values[0]
                )
            })?;
            Ok(Some(value))
        } else {
            match policy.duplicate_singleton {
                RequestDuplicatSingletonPolicy::Reject => {
                    anyhow::bail!("Content-Length is duplicated in request headers")
                }
                RequestDuplicatSingletonPolicy::First => {
                    let value = values.first().unwrap();
                    let value = value.parse::<u64>().with_context(|| {
                        format!(
                            "unable to convert Content-Length value \"{}\" to u64",
                            value
                        )
                    })?;
                    Ok(Some(value))
                }
                RequestDuplicatSingletonPolicy::Last => {
                    let value = values.last().unwrap();
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

impl Default for RequestDuplicatSingletonPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

fn parse_start(line: &ByteStr<false>) -> anyhow::Result<(Method, &ByteStr<true>, Version)> {
    let mut parts = line.split_pat_naive(byte_str!(i:b" "));
    let method = parts.next().unwrap().try_into()?;
    let path = parts.next().context("missing path")?.case_sensitive();
    let version = if let Some(version) = parts.next() {
        version.try_into()?
    } else {
        Version::Http09
    };
    Ok((method, path, version))
}

pub struct Response {
    pub version: Version,
    pub status: Status,
    pub headers: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}

impl TryFrom<&Response> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(
        Response {
            version,
            status,
            headers,
            body,
        }: &Response,
    ) -> Result<Self, Self::Error> {
        let mut buf = Vec::with_capacity(body.len() + 512);
        let version_str: &str = (*version).into();
        buf.extend(version_str.bytes());
        buf.push(b' ');
        let status_str: &str = (*status).into();
        buf.extend(status_str.bytes());
        buf.extend(b"\r\n");
        for (k, v) in headers {
            if !k.is_ascii() {
                anyhow::bail!("key \"{k}\" contains non-ASCII character(s)")
            }
            for v in v {
                if !v.is_ascii() {
                    anyhow::bail!("value \"{v}\" contains non-ASCII character(s)")
                }
                buf.extend(k.trim().bytes());
                buf.extend(b": ");
                buf.extend(v.trim().bytes());
                buf.extend(b"\r\n");
            }
        }
        Ok(buf)
    }
}

impl Response {
    pub fn check_sanity(&self) -> anyhow::Result<()> {
        let Response {
            version,
            status,
            headers,
            body
        } = self;
        if matches!(version, Version::Http09) {
            if !headers.is_empty() {
                anyhow::bail!("original HTTP does not support response headers")
            }
        }
        Ok(())
    }
}