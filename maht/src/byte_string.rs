use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[repr(transparent)]
#[derive(Clone)]
pub struct ByteString<const cs: bool>(Vec<u8>);

#[repr(transparent)]
pub struct ByteStr<const cs: bool>([u8]);

pub trait ToByteString: Sized {
    fn to_byte_string<const cs: bool>(self) -> ByteString<cs>;

    fn case_sensitive(self) -> ByteString<true> {
        self.to_byte_string::<true>()
    }

    fn case_insensitive(self) -> ByteString<false> {
        self.to_byte_string::<false>()
    }
}

impl ToByteString for Vec<u8> {
    fn to_byte_string<const cs: bool>(self) -> ByteString<cs> {
        ByteString(self)
    }
}

pub trait ToByteStr {
    fn to_byte_str<const cs: bool>(&self) -> &ByteStr<cs>;

    fn case_sensitive(&self) -> &ByteStr<true> {
        self.to_byte_str::<true>()
    }

    fn case_insensitive(&self) -> &ByteStr<false> {
        self.to_byte_str::<false>()
    }
}

impl ToByteStr for [u8] {
    fn to_byte_str<const cs: bool>(&self) -> &ByteStr<cs> {
        unsafe { &*(self as *const [u8] as *const ByteStr<cs>) }
    }
}

impl <const cs: bool> Borrow<ByteStr<cs>> for [u8] {
    fn borrow(&self) -> &ByteStr<cs> {
        self.to_byte_str()
    }
}

impl <const cs: bool> AsRef<ByteStr<cs>> for [u8] {
    fn as_ref(&self) -> &ByteStr<cs> {
        self.to_byte_str()
    }
}

impl <const cs: bool, const n: usize> Borrow<ByteStr<cs>> for [u8; n] {
    fn borrow(&self) -> &ByteStr<cs> {
        self.to_byte_str()
    }
}

impl <const cs: bool, const n: usize> AsRef<ByteStr<cs>> for [u8; n] {
    fn as_ref(&self) -> &ByteStr<cs> {
        self.to_byte_str()
    }
}

impl <const cs: bool> Deref for ByteString<cs> {
    type Target = ByteStr<cs>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*((&self.0 as &[u8]) as *const [u8] as *const ByteStr<cs>) }
    }
}

impl <const cs: bool> Borrow<ByteStr<cs>> for ByteString<cs> {
    fn borrow(&self) -> &ByteStr<cs> {
        self.deref()
    }
}

impl <const cs: bool> AsRef<ByteStr<cs>> for ByteString<cs> {
    fn as_ref(&self) -> &ByteStr<cs> {
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

impl <const cs: bool> ByteStr<cs> where ByteStr<cs>: Eq {
    pub fn split_pat_naive<'a>(&'a self, pat: &'a Self) -> ByteStrSplitPatNaiveIterator<'a, cs> {
        ByteStrSplitPatNaiveIterator { string: self, pat }
    }
    
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.0)
    }
    
    pub fn to_owned(&self) -> ByteString<cs> {
        ByteString(self.0.to_vec())
    }
}

pub struct ByteStrSplitPatNaiveIterator<'a, const cs: bool> where ByteStr<cs>: Eq {
    string: &'a ByteStr<cs>,
    pat: &'a ByteStr<cs>
}

impl <'a, const cs: bool> Iterator for ByteStrSplitPatNaiveIterator<'a, cs>  where ByteStr<cs>: Eq {
    type Item = &'a ByteStr<cs>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.string.0.len() == 0 {
            return None
        }
        let i = self.string.0.windows(self.pat.0.len())
            .position(|x| x.to_byte_str::<cs>() == self.pat)
            .unwrap_or(self.string.0.len());
        let ret = self.string.0[..i].to_byte_str();
        self.string = self.string.0[(i + self.pat.0.len())..].to_byte_str();
        Some(ret)
    }
}

pub const fn make_const_byte_str<const cs: bool>(slice: &'static [u8]) -> &'static ByteStr<cs> {
    unsafe { &*(slice as *const [u8] as *const ByteStr<cs>) }
}

#[macro_export]
macro_rules! byte_str {
    (s:$lit:literal) => {
        $crate::byte_string::make_const_byte_str($lit)
    };
    (i:$lit:literal) => {
        $crate::byte_string::make_const_byte_str($lit)
    };
}

#[macro_export]
macro_rules! const_byte_str {
    ($name:ident = s:$val:literal) => {
        #[allow(non_upper_case_globals)]
        const $name: &$crate::byte_string::ByteStr<true> = $crate::byte_string::make_const_byte_str($val);
    };
    ($name:ident = i:$val:literal) => {
        #[allow(non_upper_case_globals)]
        const $name: &$crate::byte_string::ByteStr<false> = $crate::byte_string::make_const_byte_str($val);
    };
    (pub $name:ident = s:$val:literal) => {
        pub const $name: &$crate::byte_string::ByteStr<true> = $crate::byte_string::make_const_byte_str($val);
    };
    (pub $name:ident = i:$val:literal) => {
        pub const $name: &$crate::byte_string::ByteStr<false> = $crate::byte_string::make_const_byte_str($val);
    };
}