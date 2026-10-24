use crate::error::Error;

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encodes bytes as a base64 string.
pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);

        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        out.push(ALPHABET[((n >> 18) & 0x3f) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[((n >> 6) & 0x3f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(n & 0x3f) as usize] as char } else { '=' });
    }
    out
}

fn char_value(c: u8) -> Option<u32> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as u32),
        b'a'..=b'z' => Some((c - b'a') as u32 + 26),
        b'0'..=b'9' => Some((c - b'0') as u32 + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// Decodes a base64 string; errors on invalid characters or bad padding.
pub fn base64_decode(data: &str) -> Result<Vec<u8>, Error> {
    let clean: Vec<u8> = data
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r')
        .collect();
    if !clean.len().is_multiple_of(4) {
        return Err(Error::InvalidLength { base: "base64" });
    }
    let pad_total = clean.iter().rev().take_while(|&&b| b == b'=').count();
    if pad_total > 2 || clean[..clean.len() - pad_total].contains(&b'=') {
        return Err(Error::InvalidPadding);
    }

    let mut out = Vec::with_capacity((clean.len() / 4) * 3);
    for chunk in clean.chunks(4) {
        let pad = chunk.iter().filter(|&&b| b == b'=').count();
        let mut n: u32 = 0;
        for &b in chunk {
            n <<= 6;
            if b == b'=' {
                continue;
            }
            n |= char_value(b).ok_or(Error::InvalidCharacter { base: "base64", ch: b as char })?;
        }
        let bytes = n.to_be_bytes();
        match pad {
            0 => out.extend_from_slice(&bytes[1..4]),
            1 => out.extend_from_slice(&bytes[1..3]),
            2 => out.extend_from_slice(&bytes[1..2]),
            _ => return Err(Error::InvalidPadding),
        }
    }
    Ok(out)
}

/// Returns true if the input is valid base64.
pub fn base64_check(data: &str) -> bool {
    base64_decode(data).is_ok()
}

/// Strip invalid characters, repair padding, decode; returns (base,clean).
pub fn base64_decode_fix(data: &str) -> (Result<Vec<u8>, Error>, String) {
    let mut clean: String = data
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '+' || c == '/')
        .collect();
    let rem = clean.len() % 4;
    if rem != 0 {
        clean.extend(std::iter::repeat_n('=', 4 - rem));
    }
    (base64_decode(&clean), clean)
}
