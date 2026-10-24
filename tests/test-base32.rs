#[cfg(test)]
mod tests {
    use base::base32::*;

    #[test]
    fn base32_base32_encode_basic() {
        assert_eq!(base32_encode(b"MAN123"), "JVAU4MJSGM======");
        assert_eq!(base32_encode(b"MAN12"), "JVAU4MJS");
        assert_eq!(base32_encode(b"MAN1"), "JVAU4MI=");
        assert_eq!(base32_encode(b"MAN"), "JVAU4===");
        assert_eq!(base32_encode(b"Ma"), "JVQQ====");
        assert_eq!(base32_encode(b"M"), "JU======");
        assert_eq!(base32_encode(b""), "");
    }

    #[test]
    fn base32_decode_basic() {
        assert_eq!(base32_decode("K5HU2QKOHE======").unwrap(), b"WOMAN9");
        assert_eq!(base32_decode("K5HU2QKO").unwrap(), b"WOMAN");
        assert_eq!(base32_decode("K5HU2QI=").unwrap(), b"WOMA");
        assert_eq!(base32_decode("K5HU2===").unwrap(), b"WOM");
        assert_eq!(base32_decode("K5HQ====").unwrap(), b"WO");
        assert_eq!(base32_decode("K4======").unwrap(), b"W");
        assert_eq!(base32_decode("").unwrap(), b"");
    }

    #[test]
    fn base32_roundtrip() {
        let data = b"The quick brown fox jumps over the lazy dog";
        assert_eq!(base32_decode(&base32_encode(data)).unwrap(), data);
    }

    #[test]
    fn base32_decode_invalid_char_errs() {
        assert!(base32_decode("!!!!!!!=").is_err());
        assert!(base32_decode("11111111").is_err());
        assert!(base32_decode("00000000").is_err());
    }
    #[test]
    fn base32_check_valid_and_invalid() {
        assert!(base32_check("KJKVGVBT"));
        assert!(base32_check("KJKVGVA="));
        assert!(!base32_check("NOT base 32"));
        assert!(!base32_check("ONSHG="));
        assert!(!base32_check("ONSHG5=="));
    }
    #[test]
    fn base32_decode_fix_cleans_input() {
        assert!(base32_decode_fix("K5HU2QKOHE======").0.is_ok());
        assert!(base32_decode_fix("K5HU2QKO").0.is_ok());
        assert!(base32_decode_fix("K5HU2QI=").0.is_ok());
        assert!(base32_decode_fix("K5HU2===").0.is_ok());
        assert!(base32_decode_fix("K5HQ====").0.is_ok());
        assert!(base32_decode_fix("K4======").0.is_ok());
        assert!(base32_decode_fix("").0.is_ok());
        assert!(base32_decode_fix("fnd32").0.is_ok());
        assert!(base32_decode("K5HU2===K5HU2QKO").is_err());
        assert!(base32_decode("K=HU2QKO").is_err());
    }
}
