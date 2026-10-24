use crate::error::Error;

const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Encodes bytes as a base58 string.
pub fn base58_encode(data: &[u8]) -> String {
    let zeros = data.iter().take_while(|&&b| b == 0).count();
    let mut digits: Vec<u8> = Vec::with_capacity(data.len() * 138 / 100 + 1);
    for &byte in &data[zeros..] {
        let mut carry = byte as u32;
        for d in digits.iter_mut() {
            carry += (*d as u32) << 8;
            *d = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }
    let mut out = String::with_capacity(zeros + digits.len());
    out.extend(std::iter::repeat_n('1', zeros));
    out.extend(digits.iter().rev().map(|&d| ALPHABET[d as usize] as char));
    out
}

fn char_value(c: u8) -> Option<u32> {
    ALPHABET.iter().position(|&a| a == c).map(|i| i as u32)
}

/// Decodes a base58 string; errors on invalid characters.
pub fn base58_decode(data: &str) -> Result<Vec<u8>, Error> {
    let clean: Vec<u8> = data
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r')
        .collect();
    let zeros = clean.iter().take_while(|&&b| b == b'1').count();

    let mut bytes: Vec<u8> = Vec::with_capacity(clean.len() * 733 / 1000 + 1);
    for &c in &clean[zeros..] {
        let mut carry = char_value(c).ok_or(Error::InvalidCharacter { base: "base58", ch: c as char })?;
        for b in bytes.iter_mut() {
            carry += (*b as u32) * 58;
            *b = (carry & 0xff) as u8;
            carry >>= 8;
        }
        while carry > 0 {
            bytes.push((carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    let mut out = vec![0u8; zeros];
    out.extend(bytes.iter().rev());
    Ok(out)
}

/// Returns true if the input is valid base58.
pub fn base58_check(data: &str) -> bool {
    base58_decode(data).is_ok()
}

/// Strip invalid characters, repair padding, decode; returns (base,clean).
pub fn base58_decode_fix(data: &str) -> (Result<Vec<u8>, Error>, String) {
    let clean: String = data
        .bytes()
        .filter(|&b| char_value(b).is_some())
        .map(|b| b as char)
        .collect();
    (base58_decode(&clean), clean)
}
