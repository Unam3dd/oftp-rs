use super::*;

mod encode_tests {
    use super::*;

    #[test]
    fn encode_wire() {
        let mut secd = Secd;
        let wire = secd.encode().expect("encode");
        assert_eq!(wire, [SECDCMD]);
        assert_eq!(wire.len(), SECD_LEN);
    }
}

mod decode_tests {
    use super::*;

    #[test]
    fn decode_ok() {
        let mut secd = Secd;
        assert!(secd.decode(&[SECDCMD]).is_ok());
    }

    #[test]
    fn decode_bad_command() {
        let mut secd = Secd;
        let err = secd.decode(&[b'X']).unwrap_err();
        assert_eq!(err, SecdError::BadCommandError);
    }

    #[test]
    fn decode_bad_len_empty() {
        let mut secd = Secd;
        let err = secd.decode(&[]).unwrap_err();
        assert_eq!(err, SecdError::InvalidSecdSizeError);
    }

    #[test]
    fn decode_bad_len_too_long() {
        let mut secd = Secd;
        let err = secd.decode(&[SECDCMD, 0x00]).unwrap_err();
        assert_eq!(err, SecdError::InvalidSecdSizeError);
    }
}

mod roundtrip_tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let mut original = Secd;
        let wire = original.encode().expect("encode");

        let mut decoded = Secd;
        decoded.decode(&wire).expect("decode");
        assert_eq!(decoded, original);
    }
}
