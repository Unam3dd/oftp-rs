use super::*;

mod encode_tests {
    use super::*;

    #[test]
    fn encode_normal() {
        let wire = Esid::normal().encode().unwrap();
        assert_eq!(wire.len(), ESID_MIN_WIRE_LEN);
        assert_eq!(
            wire,
            vec![ESIDCMD, b'0', b'0', b'0', b'0', b'0', ESID_CR]
        );
    }

    #[test]
    fn encode_with_reason_and_text() {
        let esid = Esid {
            reason: 2,
            reason_text: b"KO".to_vec(),
            cr: ESID_CR,
        };
        let wire = esid.encode().unwrap();
        assert_eq!(wire.len(), esid_wire_len(2));
        assert_eq!(&wire[..ESID_HEADER_LEN], &[ESIDCMD, b'0', b'2', b'0', b'0', b'2']);
        assert_eq!(&wire[ESID_TEXT_OFFSET..ESID_TEXT_OFFSET + 2], b"KO");
        assert_eq!(wire.last().copied(), Some(ESID_CR));
    }

    #[test]
    fn encode_cr_alt() {
        let esid = Esid {
            reason: ESID_REASON_NORMAL,
            reason_text: Vec::new(),
            cr: ESID_CR_ALT,
        };
        let wire = esid.encode().unwrap();
        assert_eq!(wire.last().copied(), Some(ESID_CR_ALT));
    }

    #[test]
    fn encode_text_too_long() {
        let esid = Esid {
            reason: ESID_REASON_NORMAL,
            reason_text: vec![b'X'; ESID_TEXT_MAX + 1],
            cr: ESID_CR,
        };
        let err = esid.encode().unwrap_err();
        assert!(matches!(err, EsidError::InvalidSizeError));
    }
}

mod decode_tests {
    use super::*;

    fn normal_wire() -> Vec<u8> {
        vec![ESIDCMD, b'0', b'0', b'0', b'0', b'0', ESID_CR]
    }

    #[test]
    fn decode_normal() {
        let mut esid = Esid::default();
        esid.decode(&normal_wire()).unwrap();
        assert_eq!(esid.reason, ESID_REASON_NORMAL);
        assert!(esid.reason_text.is_empty());
        assert_eq!(esid.cr, ESID_CR);
    }

    #[test]
    fn decode_with_reason_text() {
        let wire = vec![
            ESIDCMD, b'0', b'2', b'0', b'0', b'2', b'K', b'O', ESID_CR,
        ];
        let mut esid = Esid::default();
        esid.decode(&wire).unwrap();
        assert_eq!(esid.reason, 2);
        assert_eq!(esid.reason_text, b"KO");
        assert_eq!(esid.cr, ESID_CR);
    }

    #[test]
    fn decode_bad_command() {
        let mut buf = normal_wire();
        buf[0] = b'Z';
        let mut esid = Esid::default();
        let err = esid.decode(&buf).unwrap_err();
        assert!(matches!(err, EsidError::BadCommandError));
    }

    #[test]
    fn decode_buffer_too_short() {
        let mut esid = Esid::default();
        let err = esid.decode(&[ESIDCMD, b'0', b'0']).unwrap_err();
        assert!(matches!(err, EsidError::InvalidSizeError));
    }

    #[test]
    fn decode_length_mismatch() {
        let mut buf = normal_wire();
        buf.push(b'X');
        let mut esid = Esid::default();
        let err = esid.decode(&buf).unwrap_err();
        assert!(matches!(err, EsidError::DescriptionLengthMismatch));
    }

    #[test]
    fn decode_bad_cr() {
        let mut buf = normal_wire();
        *buf.last_mut().unwrap() = 0xAA;
        let mut esid = Esid::default();
        let err = esid.decode(&buf).unwrap_err();
        assert!(matches!(err, EsidError::BadControlReturnError));
    }

    #[test]
    fn decode_cr_alt() {
        let mut buf = normal_wire();
        *buf.last_mut().unwrap() = ESID_CR_ALT;
        let mut esid = Esid::default();
        esid.decode(&buf).unwrap();
        assert_eq!(esid.cr, ESID_CR_ALT);
    }
}

mod roundtrip_tests {
    use super::*;

    #[test]
    fn roundtrip_normal() {
        let original = Esid::normal();
        let wire = original.encode().unwrap();

        let mut decoded = Esid::default();
        decoded.decode(&wire).unwrap();

        assert_eq!(decoded.reason, original.reason);
        assert_eq!(decoded.reason_text, original.reason_text);
        assert_eq!(decoded.cr, original.cr);
    }

    #[test]
    fn roundtrip_with_reason_text() {
        let original = Esid {
            reason: 12,
            reason_text: b"session closed".to_vec(),
            cr: ESID_CR_ALT,
        };
        let wire = original.encode().unwrap();

        let mut decoded = Esid::default();
        decoded.decode(&wire).unwrap();

        assert_eq!(decoded.reason, original.reason);
        assert_eq!(decoded.reason_text, original.reason_text);
        assert_eq!(decoded.cr, original.cr);
    }

    #[test]
    fn roundtrip_decode_encode_decode() {
        let wire = Esid::normal().encode().unwrap();

        let mut mid = Esid::default();
        mid.decode(&wire).unwrap();
        let again = mid.encode().unwrap();
        assert_eq!(again, wire);

        let mut end = Esid::default();
        end.decode(&again).unwrap();
        assert_eq!(end.reason, ESID_REASON_NORMAL);
        assert_eq!(end.cr, ESID_CR);
    }
}
