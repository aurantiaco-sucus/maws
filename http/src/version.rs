use crate::{AsCaseInsensitive, CaseInsensitiveStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Version {
    Http10,
    Http11,
    Http20,
}

pub mod version_names {
    pub const HTTP10: &str = "HTTP/1.0";
    pub const HTTP11: &str = "HTTP/1.1";
    pub const HTTP20: &str = "HTTP/2.0";
}

impl Version {
    pub fn parse(value: &CaseInsensitiveStr) -> Option<Self> {
        if value == version_names::HTTP10.case_insensitive() {
            Some(Self::Http10)
        } else if value == version_names::HTTP11.case_insensitive() {
            Some(Self::Http11)
        } else if value == version_names::HTTP20.case_insensitive() {
            Some(Self::Http20)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Version::Http10 => version_names::HTTP10,
            Version::Http11 => version_names::HTTP11,
            Version::Http20 => version_names::HTTP20,
        }
    }
}