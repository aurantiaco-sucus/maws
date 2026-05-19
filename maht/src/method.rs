use crate::{const_byte_str, ByteStr};

const_byte_str!(GET = i:b"GET");
const_byte_str!(HEAD = i:b"HEAD");
const_byte_str!(POST = i:b"POST");
const_byte_str!(PUT = i:b"PUT");
const_byte_str!(DELETE = i:b"DELETE");
const_byte_str!(CONNECT = i:b"CONNECT");
const_byte_str!(OPTIONS = i:b"OPTIONS");
const_byte_str!(TRACE = i:b"TRACE");
const_byte_str!(PATCH = i:b"PATCH");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

impl TryFrom<&ByteStr<false>> for Method {
    type Error = anyhow::Error;
    fn try_from(value: &ByteStr<false>) -> Result<Self, Self::Error> {
        if value == GET {
            Ok(Method::GET)
        } else if value == HEAD {
            Ok(Method::HEAD)
        } else if value == POST {
            Ok(Method::POST)
        } else if value == PUT {
            Ok(Method::PUT)
        } else if value == DELETE {
            Ok(Method::DELETE)
        } else if value == CONNECT {
            Ok(Method::CONNECT)
        } else if value == OPTIONS {
            Ok(Method::OPTIONS)
        } else if value == TRACE {
            Ok(Method::TRACE)
        } else if value == PATCH {
            Ok(Method::PATCH)
        } else {
            Err(anyhow::anyhow!("unknown method"))
        }
    }
}

impl From<Method> for &'static ByteStr<false> {
    fn from(value: Method) -> Self {
        match value {
            Method::GET => GET,
            Method::HEAD => HEAD,
            Method::POST => POST,
            Method::PUT => PUT,
            Method::DELETE => DELETE,
            Method::CONNECT => CONNECT,
            Method::OPTIONS => OPTIONS,
            Method::TRACE => TRACE,
            Method::PATCH => PATCH,
        }
    }
}