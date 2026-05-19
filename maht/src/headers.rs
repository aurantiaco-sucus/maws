use crate::util::ByteSliceExt;
use crate::{ByteStr, ByteString, ToByteStr, ToByteString};
use std::collections::HashMap;

#[derive(Default)]
#[derive(Clone, Eq, PartialEq)]
pub struct Headers(HashMap<ByteString<false>, Vec<ByteString<true>>>);

#[derive(Default, Clone)]
pub struct HeaderPolicy {
    allow_untrimmed_key: bool,
}

impl Headers {
    pub fn parse(buf: &[u8], HeaderPolicy { allow_untrimmed_key }: &HeaderPolicy) -> anyhow::Result<Self> {
        let mut map: HashMap<ByteString<false>, Vec<ByteString<true>>> = HashMap::new();
        let mut prev = 0;
        for i in buf.find_pattern_all_naive(b"\r\n") {
            let line = &buf[prev..i];
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
            prev = i + 2;
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
}