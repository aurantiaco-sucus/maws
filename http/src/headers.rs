use crate::{AsCaseInsensitive, CaseInsensitiveStr, CaseInsensitiveString};
use anyhow::Context;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct Headers(HashMap<CaseInsensitiveString, Vec<String>>);

#[derive(Default, Clone)]
pub struct HeaderPolicy {
    allow_untrimmed_key: bool,
}

impl Headers {
    pub fn parse(buf: &str, HeaderPolicy { allow_untrimmed_key }: &HeaderPolicy) -> anyhow::Result<Self> {
        let mut map: HashMap<CaseInsensitiveString, Vec<String>> = HashMap::new();
        for line in buf.lines() {
            let (mut key, value) = line.split_once(':')
                .with_context(|| format!("malformed header line: {line}"))?;
            if key.trim_ascii().len() != key.len() {
                if !allow_untrimmed_key {
                    anyhow::bail!("header key is untrimmed")
                }
                key = key.trim_ascii();
            }
            map.entry(key.to_owned().case_insensitive())
                .or_default()
                .push(value.to_owned());
        }
        Ok(Headers(map))
    }

    pub fn get(&self, key: &CaseInsensitiveStr) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item=(&CaseInsensitiveString, &Vec<String>)> {
        self.0.iter()
    }

    pub fn insert(&mut self, key: &CaseInsensitiveStr, value: impl Into<String>) {
        let entry = if let Some(x) = self.0.get_mut(key) {
            x
        } else {
            self.0.insert(key.to_owned(), Vec::new());
            self.0.get_mut(key).unwrap()
        };
        entry.push(value.into());
    }
}