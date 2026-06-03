use crate::{AsCaseInsensitive, CaseInsensitiveStr};

pub mod method_names {
    pub const GET: &str = "GET";
    pub const HEAD: &str = "HEAD";
    pub const POST: &str = "POST";
    pub const PUT: &str = "PUT";
    pub const DELETE: &str = "DELETE";
    pub const CONNECT: &str = "CONNECT";
    pub const OPTIONS: &str = "OPTIONS";
    pub const TRACE: &str = "TRACE";
    pub const PATCH: &str = "PATCH";
}

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

impl Method {
    pub fn parse(value: &CaseInsensitiveStr) -> Option<Self> {
        if value == method_names::GET.case_insensitive() {
            Some(Self::GET)
        } else if value == method_names::HEAD.case_insensitive() {
            Some(Self::HEAD)
        } else if value == method_names::POST.case_insensitive() {
            Some(Self::POST)
        } else if value == method_names::PUT.case_insensitive() {
            Some(Self::PUT)
        } else if value == method_names::DELETE.case_insensitive() {
            Some(Self::DELETE)
        } else if value == method_names::CONNECT.case_insensitive() {
            Some(Self::CONNECT)
        } else if value == method_names::OPTIONS.case_insensitive() {
            Some(Self::OPTIONS)
        } else if value == method_names::TRACE.case_insensitive() {
            Some(Self::TRACE)
        } else if value == method_names::PATCH.case_insensitive() {
            Some(Self::PATCH)
        } else {
            None
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            Self::GET => method_names::GET,
            Self::HEAD => method_names::HEAD,
            Self::POST => method_names::POST,
            Self::PUT => method_names::PUT,
            Self::DELETE => method_names::DELETE,
            Self::CONNECT => method_names::CONNECT,
            Self::OPTIONS => method_names::OPTIONS,
            Self::TRACE => method_names::TRACE,
            Self::PATCH => method_names::PATCH,
        }
    }
}