use std::collections::HashMap;
use anyhow::Context;
use maws_http::url_decode;

pub fn parse(raw: &str) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let mut map = HashMap::new();
    for phrase in raw.split('&') {
        let phrase = phrase.trim();
        if phrase.is_empty() {
            continue
        }
        let (key, value) = phrase.split_once('=')
            .with_context(|| format!("invalid query phrase: {phrase}"))?;
        let key = url_decode(key).context("error decoding query key")?;
        let value = url_decode(value).context("error decoding query value")?;
        map.entry(key).or_default().push(value);
    }
    Ok(map)
}