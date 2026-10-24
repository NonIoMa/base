#[cfg(test)]
mod tests {
    use base::base58::*;

    #[test]
    fn base58_encode_basic() {
        assert_eq!(base58_encode(b"a"), "2g");
        assert_eq!(base58_encode(b"bbb"), "a3gV");
        assert_eq!(base58_encode(b"ccc"), "aPEr");
        assert_eq!(base58_encode(b"Hello World!"), "2NEpo7TZRRrLZSi2U");
        assert_eq!(base58_encode(&[0, 0, 0, 0x28, 0x7f, 0xb4, 0xcd]), "111233QC4");
        assert_eq!(base58_encode(&[0, 0, 0]), "111");
        assert_eq!(base58_encode(b""), "");
    }

    #[test]
    fn base58_decode_basic() {
        assert_eq!(base58_decode("2g").unwrap(), b"a");
        assert_eq!(base58_decode("a3gV").unwrap(), b"bbb");
        assert_eq!(base58_decode("aPEr").unwrap(), b"ccc");
        assert_eq!(base58_decode("2NEpo7TZRRrLZSi2U").unwrap(), b"Hello World!");
        assert_eq!(
            base58_decode("111233QC4").unwrap(),
            vec![0, 0, 0, 0x28, 0x7f, 0xb4, 0xcd]
        );
        assert_eq!(base58_decode("111").unwrap(), vec![0, 0, 0]);
        assert_eq!(base58_decode("").unwrap(), b"");
    }

    #[test]
    fn base58_roundtrip() {
        let data = b"The quick brown fox jumps over the lazy dog";
        assert_eq!(base58_decode(&base58_encode(data)).unwrap(), data);
        let bin: Vec<u8> = (0..=255).collect();
        assert_eq!(base58_decode(&base58_encode(&bin)).unwrap(), bin);
        let lead: Vec<u8> = vec![0, 0, 1, 2, 3, 0, 0];
        assert_eq!(base58_decode(&base58_encode(&lead)).unwrap(), lead);
    }

    #[test]
    fn base58_decode_invalid_char_errs() {
        assert!(base58_decode("0").is_err());
        assert!(base58_decode("O").is_err());
        assert!(base58_decode("I").is_err());
        assert!(base58_decode("l").is_err());
        assert!(base58_decode("2g=").is_err());
        assert!(base58_decode("!!!!").is_err());
    }

    #[test]
    fn base58_check_valid_and_invalid() {
        assert!(base58_check("2g"));
        assert!(base58_check("111233QC4"));
        assert!(base58_check(""));
        assert!(!base58_check("NOT base 58"));
        assert!(!base58_check("0OIl"));
    }

    #[test]
    fn base58_decode_fix_cleans_input() {
        assert!(base58_decode_fix("2g").0.is_ok());
        assert!(base58_decode_fix("").0.is_ok());
        assert!(base58_decode_fix("2g\n").0.is_ok());
        assert!(base58_decode_fix("0O2gIl").0.is_ok());
        assert_eq!(base58_decode_fix("0O2gIl").1, "2g");
    }
}