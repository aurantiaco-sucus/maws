use anyhow::Context;

pub fn url_decode(src: &str) -> anyhow::Result<String> {
    let mut result = Vec::new();
    let mut iter = src.bytes();
    loop {
        let ch = if let Some(ch) = iter.next() { ch } else {
            break
        };
        if ch != b'%' {
            result.push(ch);
        }
        let b1 = iter.next()
            .with_context(|| format!("invalid encoded URL: {src}"))?;
        let b2 = iter.next()
            .with_context(|| format!("invalid encoded URL: {src}"))?;
        result.push(parse_percent_code(b1, b2)?);
    }
    String::from_utf8(result)
        .context("invalid UTF-8")
}

fn parse_percent_code(b1: u8, b2: u8) -> anyhow::Result<u8> {
    Ok(parse_percent_digit(b1)? << 4 + parse_percent_digit(b2)?)
}

fn parse_percent_digit(b: u8) -> anyhow::Result<u8> {
    Ok(match b {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'a' | b'A' => 0xA,
        b'b' | b'B' => 0xB,
        b'c' | b'C' => 0xC,
        b'd' | b'D' => 0xD,
        b'e' | b'E' => 0xE,
        b'f' | b'F' => 0xF,
        _ => anyhow::bail!("invalid hexadecimal digit: {b}")
    })
}