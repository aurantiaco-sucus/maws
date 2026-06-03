use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Default, Debug, Clone)]
#[repr(transparent)]
pub struct CaseInsensitiveString(String);

#[derive(Debug)]
#[repr(transparent)]
pub struct CaseInsensitiveStr(str);

impl PartialEq for CaseInsensitiveStr {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for CaseInsensitiveStr {}

impl PartialOrd for CaseInsensitiveStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CaseInsensitiveStr {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_chars = self.0.chars();
        let mut other_chars = other.0.chars();
        loop {
            return match (self_chars.next(), other_chars.next()) {
                (Some(ch_self), Some(ch_other)) =>
                    match ch_self.to_ascii_lowercase().cmp(&ch_other.to_ascii_lowercase()) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                        Ordering::Equal => continue
                    },
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            }
        }
    }
}

impl Hash for CaseInsensitiveStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for ch in self.0.chars() {
            ch.to_ascii_lowercase().hash(state);
        }
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        (self as &CaseInsensitiveStr) == (other as &CaseInsensitiveStr)
    }
}

impl Eq for CaseInsensitiveString {}

impl PartialOrd for CaseInsensitiveString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for CaseInsensitiveString {
    fn cmp(&self, other: &Self) -> Ordering {
        (self as &CaseInsensitiveStr).cmp(other as &CaseInsensitiveStr)
    }
}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self as &CaseInsensitiveStr).hash(state);
    }
}

pub trait AsCaseInsensitive {
    type Target;

    fn case_insensitive(self) -> Self::Target;
}

impl AsCaseInsensitive for String {
    type Target = CaseInsensitiveString;

    fn case_insensitive(self) -> Self::Target {
        CaseInsensitiveString(self)
    }
}

impl <'a> AsCaseInsensitive for &'a str {
    type Target = &'a CaseInsensitiveStr;

    fn case_insensitive(self) -> Self::Target {
        unsafe { &*(self as *const str as *const CaseInsensitiveStr) }
    }
}

impl CaseInsensitiveString {
    pub fn case_sensitive(self) -> String {
        self.0
    }

    pub fn case_sensitive_ref(&self) -> &String {
        &self.0
    }

    pub fn case_sensitive_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl CaseInsensitiveStr {
    pub fn case_sensitive(&self) -> &str {
        &self.0
    }
    
    pub fn to_owned(&self) -> CaseInsensitiveString {
        self.0.to_owned().case_insensitive()
    }
}

impl Deref for CaseInsensitiveString {
    type Target = CaseInsensitiveStr;

    fn deref(&self) -> &Self::Target {
        self.0.as_str().case_insensitive()
    }
}

impl AsRef<CaseInsensitiveStr> for CaseInsensitiveString {
    fn as_ref(&self) -> &CaseInsensitiveStr {
        self
    }
}

impl Borrow<CaseInsensitiveStr> for CaseInsensitiveString {
    fn borrow(&self) -> &CaseInsensitiveStr {
        self
    }
}

pub trait GetWithStr<T> {
    fn get_with_str(&self, key: &str) -> Option<&T>;
    fn get_mut_with_str(&mut self, key: &mut str) -> Option<&mut T>;
}

impl <T> GetWithStr<T> for BTreeMap<CaseInsensitiveString, T> {
    fn get_with_str(&self, key: &str) -> Option<&T> {
        self.get(key.case_insensitive())
    }

    fn get_mut_with_str(&mut self, key: &mut str) -> Option<&mut T> {
        self.get_mut(key.case_insensitive())
    }
}

impl <T> GetWithStr<T> for HashMap<CaseInsensitiveString, T> {
    fn get_with_str(&self, key: &str) -> Option<&T> {
        self.get(key.case_insensitive())
    }

    fn get_mut_with_str(&mut self, key: &mut str) -> Option<&mut T> {
        self.get_mut(key.case_insensitive())
    }
}