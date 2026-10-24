use crate::error::Error;

const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/// Encodes bytes as a base32 string.
pub fn base32_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(5) * 8);
    for chunk in data.chunks(5) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let b3 = *chunk.get(3).unwrap_or(&0);
        let b4 = *chunk.get(4).unwrap_or(&0);

        let n =
            ((b0 as u64) << 32) |
            ((b1 as u64) << 24) |
            ((b2 as u64) << 16) |
            ((b3 as u64) << 8) |
            (b4 as u64);

        out.push(ALPHABET[((n >> 35) & 0x1f) as usize] as char);
        out.push(ALPHABET[((n >> 30) & 0x1f) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[((n >> 25) & 0x1f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 1 { ALPHABET[((n >> 20) & 0x1f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[((n >> 15) & 0x1f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 3 { ALPHABET[((n >> 10) & 0x1f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 3 { ALPHABET[((n >> 5) & 0x1f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 4 { ALPHABET[(n & 0x1f) as usize] as char } else { '=' });
    }
    out
}
fn char_value(c: u8) -> Option<u64> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as u64),
        b'2'..=b'7' => Some((c - b'0' - 2 + 26) as u64),
        _ => None,
    }
}

/// Decodes a base32 string; errors on invalid characters or bad padding.
pub fn base32_decode(data: &str) -> Result<Vec<u8>, Error> {
    let clean: Vec<u8> = data
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r')
        .collect();
    if !clean.len().is_multiple_of(8) {
        return Err(Error::InvalidLength { base: "base32" });
    }
    let pad_total = clean
        .iter()
        .rev()
        .take_while(|&&b| b == b'=')
        .count();
    if pad_total > 6 || clean[..clean.len() - pad_total].contains(&b'=') {
        return Err(Error::InvalidPadding);
    }

    let mut out = Vec::with_capacity((clean.len() / 8) * 5);
    for chunk in clean.chunks(8) {
        let pad = chunk
            .iter()
            .filter(|&&b| b == b'=')
            .count();
        let mut n: u64 = 0;
        for &b in chunk {
            n <<= 5;
            if b == b'=' {
                continue;
            }
            n |= char_value(b).ok_or(Error::InvalidCharacter { base: "base32", ch: b as char })?;
        }
        let bytes = n.to_be_bytes();
        match pad {
            0 => out.extend_from_slice(&bytes[3..8]),
            1 => out.extend_from_slice(&bytes[3..7]),
            3 => out.extend_from_slice(&bytes[3..6]),
            4 => out.extend_from_slice(&bytes[3..5]),
            6 => out.extend_from_slice(&bytes[3..4]),
            _ => {
                return Err(Error::InvalidPadding);
            }
        }
    }
    Ok(out)
}

/// Returns true if the input is valid base32.
pub fn base32_check(data: &str) -> bool {
    base32_decode(data).is_ok()
}

/// Strip invalid characters, repair padding, decode; returns (base,clean)
pub fn base32_decode_fix(data: &str) -> (Result<Vec<u8>, Error>, String) {
    let mut clean: String = data
        .to_ascii_uppercase()
        .chars()
        .filter(|&c| c.is_ascii_uppercase() || matches!(c, '2'..='7'))
        .collect();
    let rem = clean.len() % 8;
    if rem != 0 {
        clean.extend(std::iter::repeat_n('=', 8 - rem));
    }
    (base32_decode(&clean), clean)
}
