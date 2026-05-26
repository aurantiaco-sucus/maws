use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[repr(transparent)]
#[derive(Clone)]
pub struct ByteString<const CS: bool>(Vec<u8>);

#[repr(transparent)]
pub struct ByteStr<const CS: bool>([u8]);

pub trait ToByteString: Sized {
    fn to_byte_string<const CS: bool>(self) -> ByteString<CS>;

    fn case_sensitive(self) -> ByteString<true> {
        self.to_byte_string::<true>()
    }

    fn case_insensitive(self) -> ByteString<false> {
        self.to_byte_string::<false>()
    }
}

impl ToByteString for Vec<u8> {
    fn to_byte_string<const CS: bool>(self) -> ByteString<CS> {
        ByteString(self)
    }
}

pub trait ToByteStr {
    fn to_byte_str<const CS: bool>(&self) -> &ByteStr<CS>;

    fn case_sensitive(&self) -> &ByteStr<true> {
        self.to_byte_str::<true>()
    }

    fn case_insensitive(&self) -> &ByteStr<false> {
        self.to_byte_str::<false>()
    }
}

impl ToByteStr for [u8] {
    fn to_byte_str<const CS: bool>(&self) -> &ByteStr<CS> {
        unsafe { &*(self as *const [u8] as *const ByteStr<CS>) }
    }
}

impl <const CS: bool> Borrow<ByteStr<CS>> for [u8] {
    fn borrow(&self) -> &ByteStr<CS> {
        self.to_byte_str()
    }
}

impl <const CS: bool> AsRef<ByteStr<CS>> for [u8] {
    fn as_ref(&self) -> &ByteStr<CS> {
        self.to_byte_str()
    }
}

impl <const CS: bool, const N: usize> Borrow<ByteStr<CS>> for [u8; N] {
    fn borrow(&self) -> &ByteStr<CS> {
        self.to_byte_str()
    }
}

impl <const CS: bool, const N: usize> AsRef<ByteStr<CS>> for [u8; N] {
    fn as_ref(&self) -> &ByteStr<CS> {
        self.to_byte_str()
    }
}

impl <const CS: bool> Deref for ByteString<CS> {
    type Target = ByteStr<CS>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*((&self.0 as &[u8]) as *const [u8] as *const ByteStr<CS>) }
    }
}

impl <const CS: bool> Borrow<ByteStr<CS>> for ByteString<CS> {
    fn borrow(&self) -> &ByteStr<CS> {
        self.deref()
    }
}

impl <const CS: bool> AsRef<ByteStr<CS>> for ByteString<CS> {
    fn as_ref(&self) -> &ByteStr<CS> {
        self.deref()
    }
}

impl PartialEq for ByteStr<true> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for ByteStr<true> {}

impl PartialEq for ByteStr<false> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for ByteStr<false> {}

impl PartialEq for ByteString<true> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl Eq for ByteString<true> {}

impl PartialEq for ByteString<false> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl Eq for ByteString<false> {}

impl Hash for ByteStr<true> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0)
    }
}

impl Hash for ByteStr<false> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in &self.0 {
            state.write_u8(b.to_ascii_lowercase())
        }
    }
}

impl Hash for ByteString<true> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl Hash for ByteString<false> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl <const CS: bool> Debug for ByteStr<CS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(self.0.as_ref()) {
            Ok(val) => write!(f, "{val}"),
            Err(err) => write!(f, "(string is invalid UTF-8: {err})")
        }
    }
}

impl ByteString<true> {
    pub fn case_insensitive(self) -> ByteString<false> {
        ByteString(self.0)
    }
}

impl ByteString<false> {
    pub fn case_sensitive(self) -> ByteString<true> {
        ByteString(self.0)
    }
}

impl ByteStr<true> {
    pub fn case_insensitive(&self) -> &ByteStr<false> {
        unsafe { &*(self as *const ByteStr<true> as *const ByteStr<false>) }
    }
}

impl ByteStr<false> {
    pub fn case_sensitive(&self) -> &ByteStr<true> {
        unsafe { &*(self as *const ByteStr<false> as *const ByteStr<true>) }
    }
}

impl <const CS: bool> ByteStr<CS> {
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl <const CS: bool> ByteStr<CS> where ByteStr<CS>: Eq {
    pub fn split_pat_naive<'a>(&'a self, pat: &'a Self) -> ByteStrSplitPatNaiveIterator<'a, CS> {
        ByteStrSplitPatNaiveIterator { string: self, pat }
    }
    
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.0)
    }
    
    pub fn to_owned(&self) -> ByteString<CS> {
        ByteString(self.0.to_vec())
    }
}

pub struct ByteStrSplitPatNaiveIterator<'a, const CS: bool> where ByteStr<CS>: Eq {
    string: &'a ByteStr<CS>,
    pat: &'a ByteStr<CS>
}

impl <'a, const CS: bool> Iterator for ByteStrSplitPatNaiveIterator<'a, CS>  where ByteStr<CS>: Eq {
    type Item = &'a ByteStr<CS>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.string.0.is_empty() {
            return None
        }
        let i = self.string.0.windows(self.pat.0.len())
            .position(|x| x.to_byte_str::<CS>() == self.pat)
            .unwrap_or(self.string.0.len());
        let ret = self.string.0[..i].to_byte_str();
        if i + self.pat.0.len() < self.string.0.len() {
            self.string = self.string.0[(i + self.pat.0.len())..].to_byte_str();
        } else {
            self.string = self.string.0[..0].to_byte_str();
        }
        Some(ret)
    }
}

pub const fn make_const_byte_str<const CS: bool>(slice: &'static [u8]) -> &'static ByteStr<CS> {
    unsafe { &*(slice as *const [u8] as *const ByteStr<CS>) }
}

#[macro_export]
macro_rules! byte_str {
    (s:$lit:literal) => {
        $crate::cibstr::make_const_byte_str($lit)
    };
    (i:$lit:literal) => {
        $crate::cibstr::make_const_byte_str($lit)
    };
}

#[macro_export]
macro_rules! const_byte_str {
    ($name:ident = s:$val:literal) => {
        #[allow(non_upper_case_globals)]
        const $name: &$crate::cibstr::ByteStr<true> = $crate::cibstr::make_const_byte_str($val);
    };
    ($name:ident = i:$val:literal) => {
        #[allow(non_upper_case_globals)]
        const $name: &$crate::cibstr::ByteStr<false> = $crate::cibstr::make_const_byte_str($val);
    };
    (pub $name:ident = s:$val:literal) => {
        pub const $name: &$crate::cibstr::ByteStr<true> = $crate::cibstr::make_const_byte_str($val);
    };
    (pub $name:ident = i:$val:literal) => {
        pub const $name: &$crate::cibstr::ByteStr<false> = $crate::cibstr::make_const_byte_str($val);
    };
}