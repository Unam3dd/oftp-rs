use super::*;

fn sample_packet() -> [u8; SSID_LEN] {
    let  ssid = Ssid::default();
    ssid.encode()
        .expect("sample SSID encode")
        .try_into()
        .expect("SSID length")
}

mod encode_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let ssid = Ssid::default();
        let buf = ssid.encode().unwrap();
        assert_eq!(buf.len(), SSID_LEN);
        assert_eq!(buf[0], SSIDCMD);
    }

    #[test]
    fn test_encode_bad_cr() {
        let mut ssid = Ssid::default();
        ssid.cr = 0x0A;
        let res = ssid.encode().unwrap_err();
        assert!(matches!(res, SsidError::BadControlReturnError));
    }

    #[test]
    fn test_encode_bad_buffer_size() {
        let mut ssid = Ssid::default();
        ssid.buffer_size = 50;
        let res = ssid.encode().unwrap_err();
        assert!(matches!(res, SsidError::EncodeBufferSizeError));
    }

    #[test]
    fn test_encode_bad_credit() {
        let mut ssid = Ssid::default();
        ssid.credit = 1000;
        let res = ssid.encode().unwrap_err();
        assert!(matches!(res, SsidError::EncodeCreditError));
    }
}

mod decode_tests {
    use super::*;

    #[test]
    fn test_decode_ok() {
        let mut ssid = Ssid::default();
        let buf = sample_packet();
        assert!(ssid.decode(&buf).is_ok());
    }

    #[test]
    fn test_set_code() {
        let mut ssid = Ssid::default();
        ssid.set_code("O01779122072341").unwrap();
        assert_eq!(&ssid.code[..15], b"O01779122072341");
        assert_eq!(&ssid.code[15..], [b' '; 10]);
    }

    #[test]
    fn test_set_code_too_long() {
        let mut ssid = Ssid::default();
        assert!(matches!(
            ssid.set_code("O01234567890123456789012345"),
            Err(SsidFieldError::CodeTooLong)
        ));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let mut ssid = Ssid::default();
        ssid.code = *b"O00041234TESTORG000001234";
        ssid.password = *b"SECRET  ";
        ssid.buffer_size = 4096;
        ssid.mode = SsidMode::SendOnly;
        ssid.compression = true;
        ssid.restart = true;
        ssid.credit = 100;
        ssid.auth = true;
        ssid.user_data = *b"MYDATA  ";

        let buf = ssid.encode().unwrap();
        let mut decoded = Ssid::default();
        decoded.decode(&buf).unwrap();

        assert_eq!(decoded.level, ssid.level);
        assert_eq!(decoded.code, ssid.code);
        assert_eq!(decoded.password, ssid.password);
        assert_eq!(decoded.buffer_size, ssid.buffer_size);
        assert_eq!(decoded.mode, ssid.mode);
        assert_eq!(decoded.compression, ssid.compression);
        assert_eq!(decoded.restart, ssid.restart);
        assert_eq!(decoded.special_logic, ssid.special_logic);
        assert_eq!(decoded.credit, ssid.credit);
        assert_eq!(decoded.auth, ssid.auth);
        assert_eq!(decoded.user_data, ssid.user_data);
        assert_eq!(decoded.cr, ssid.cr);
    }

    #[test]
    fn test_decode_fields() {
        let mut ssid = Ssid::default();
        let buf = sample_packet();
        ssid.decode(&buf).unwrap();

        assert_eq!(ssid.level, ProtocolLevel::Rev20);
        assert_eq!(ssid.buffer_size, 2048);
        assert_eq!(ssid.mode, SsidMode::Both);
        assert_eq!(ssid.credit, 50);
        assert!(!ssid.compression);
        assert_eq!(ssid.cr, 0x0D);
    }

    #[test]
    fn test_decode_bad_cr() {
        let mut buf = sample_packet();
        buf[60] = 0xAA;

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadControlReturnError));
    }

    #[test]
    fn test_decode_bad_len() {
        let buf = [0u8; 60];
        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::InvalidSsidSizeError));
    }

    #[test]
    fn test_decode_bad_command() {
        let mut buf = sample_packet();
        buf[0] = b'I';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadCommandError));
    }

    #[test]
    fn test_decode_bad_level() {
        let mut buf = sample_packet();
        buf[1] = b'9';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadProtocolLevelError));
    }

    #[test]
    fn test_decode_bad_mode() {
        let mut buf = sample_packet();
        buf[40] = b'X';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadModeError));
    }

    #[test]
    fn test_decode_bad_buffer_size() {
        let mut buf = sample_packet();
        buf[35..40].copy_from_slice(b"00099");

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadBufferSizeError));
    }

    #[test]
    fn test_decode_bad_yn() {
        let mut buf = sample_packet();
        buf[41] = b'X';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadYnError));
    }

    #[test]
    fn test_decode_bad_credit() {
        let mut buf = sample_packet();
        buf[44..47].copy_from_slice(b"AAA");

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadCreditError));
    }
}
