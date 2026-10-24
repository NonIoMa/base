#[cfg(test)]
mod tests {
    use base::base64::*;

    #[test]
    fn base64_encode_basic() {
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b""), "");
    }

    #[test]
    fn  base64_decode_basic() {
        assert_eq!(base64_decode("Zm9vYmFy").unwrap(), b"foobar");
        assert_eq!(base64_decode("Zm9vYmE=").unwrap(), b"fooba");
        assert_eq!(base64_decode("Zm9vYg==").unwrap(), b"foob");
        assert_eq!(base64_decode("Zm9v").unwrap(), b"foo");
        assert_eq!(base64_decode("Zm8=").unwrap(), b"fo");
        assert_eq!(base64_decode("Zg==").unwrap(), b"f");
        assert_eq!(base64_decode("").unwrap(), b"");
    }

    #[test]
    fn base64_roundtrip() {
        let data = b"The quick brown fox jumps over the lazy dog";
        assert_eq!(base64_decode(&base64_encode(data)).unwrap(), data);
        let bin: Vec<u8> = (0..=255).collect();
        assert_eq!(base64_decode(&base64_encode(&bin)).unwrap(), bin);
    }

    #[test]
    fn base64_decode_invalid_char_errs() {
        assert!(base64_decode("!!!!").is_err());
        assert!(base64_decode("Zm9*").is_err());
        assert!(base64_decode("Zm9-").is_err());
    }

    #[test]
    fn base64_decode_invalid_padding_errs() {
        assert!(base64_decode("Zg==Zm9v").is_err());
        assert!(base64_decode("Z=9v").is_err());
        assert!(base64_decode("Zm=v").is_err());
        assert!(base64_decode("====").is_err());
        assert!(base64_decode("Z===").is_err());
    }

    #[test]
    fn base64_check_valid_and_invalid() {
        assert!(base64_check("Zm9v"));
        assert!(base64_check("Zm8="));
        assert!(!base64_check("NOT base 64"));
        assert!(!base64_check("Zm8"));
        assert!(!base64_check("Zg="));
    }

    #[test]
    fn base64_decode_fix_cleans_input() {
        assert!(base64_decode_fix("Zm9vYmFy").0.is_ok());
        assert!(base64_decode_fix("Zm9vYg").0.is_ok());
        assert!(base64_decode_fix("Zm8").0.is_ok());
        assert!(base64_decode_fix("Zg").0.is_ok());
        assert!(base64_decode_fix("").0.is_ok());
        assert!(base64_decode_fix("Zm9v!!\n").0.is_ok());
        assert_eq!(base64_decode_fix("Zm9vYg").1, "Zm9vYg==");
    }
}