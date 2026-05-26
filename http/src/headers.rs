use crate::util::ByteSliceExt;
use crate::{ByteStr, ByteString, ToByteStr};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct Headers(HashMap<ByteString<false>, Vec<ByteString<true>>>);

#[derive(Default, Clone)]
pub struct HeaderPolicy {
    allow_untrimmed_key: bool,
}

impl Headers {
    pub fn parse(buf: &[u8], HeaderPolicy { allow_untrimmed_key }: &HeaderPolicy) -> anyhow::Result<Self> {
        let mut map: HashMap<ByteString<false>, Vec<ByteString<true>>> = HashMap::new();
        for line in buf.find_pattern_all_naive(b"\r\n") {
            let (mut key, value) = if let Some(i) = line.iter().position(|x| x == &b':') {
                (&line[..i], line[(i + 1)..].trim_ascii())
            } else {
                anyhow::bail!("malformed header line: \"{:?}\"", str::from_utf8(line))
            };
            if key.first().unwrap() == &b' ' || key.last().unwrap() == &b' ' {
                if !allow_untrimmed_key {
                    anyhow::bail!("header key is untrimmed")
                }
                key = key.trim_ascii();
            }
            map.entry(key.to_byte_str().to_owned())
                .or_default()
                .push(value.to_byte_str().to_owned());
        }
        Ok(Headers(map))
    }

    pub fn get(&self, key: impl AsRef<ByteStr<false>>) -> Option<Vec<&ByteStr<true>>> {
        let key = key.as_ref();
        self.0.get(key)
            .map(|x| x.iter()
                .map(|x| x.as_ref())
                .collect::<Vec<_>>())
    }

    pub fn iter(&self) -> impl Iterator<Item=(&ByteStr<false>, &[ByteString<true>])> {
        self.0.iter()
            .map(|(k, v)| (k.as_ref(), v as &[_]))
    }

    pub fn insert(&mut self, key: impl AsRef<ByteStr<false>>, value: impl Into<ByteString<true>>) {
        let key = key.as_ref();
        let entry = if let Some(x) = self.0.get_mut(key) {
            x
        } else {
            self.0.insert(key.to_owned(), Vec::new());
            self.0.get_mut(key).unwrap()
        };
        entry.push(value.into());
    }
}