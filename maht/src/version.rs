use crate::{const_byte_str, ByteStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Version {
    Http10,
    Http11,
    Http20,
}

const_byte_str!(V10 = i:b"HTTP/1.0");
const_byte_str!(V11 = i:b"HTTP/1.1");
const_byte_str!(V20 = i:b"HTTP/2.0");

impl TryFrom<&ByteStr<false>> for Version {
    type Error = anyhow::Error;
    fn try_from(value: &ByteStr<false>) -> Result<Self, Self::Error> {
        if value == V10 {
            Ok(Version::Http10)
        } else if value == V11 {
            Ok(Version::Http11)
        } else if value == V20 {
            Ok(Version::Http20)
        } else {
            anyhow::bail!("invalid HTTP version: \"{:?}\"", value.as_str())
        }
    }
}

impl From<Version> for &'static ByteStr<false> {
    fn from(value: Version) -> Self {
        match value {
            Version::Http10 => V10,
            Version::Http11 => V11,
            Version::Http20 => V20,
        }
    }
}