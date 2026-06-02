use super::*;

mod encode_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut ssrm = Ssrm { cr: 0x0D };
        assert!(ssrm.encode().is_ok());
    }

    #[test]
    fn test_encode_bad_cr() {
        let mut ssrm = Ssrm { cr: 0x0A };

        let res = ssrm.encode().unwrap_err();

        assert!(matches!(res, SsrmError::BadControlReturnError));
    }
}

mod decode_tests {
    use super::*;

    #[test]
    fn test_decode_ok() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        assert!(buf.len() == SSRM_LEN);
        assert!(ssrm.decode(&buf).is_ok());
    }

    #[test]
    fn test_decode_cr() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        assert!(buf.len() == SSRM_LEN);

        assert!(ssrm.decode(&buf).is_ok());

        assert!(ssrm.cr != 0);
    }

    #[test]
    fn test_decode_bad_cr() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0xAA
        ];

        assert!(buf.len() == SSRM_LEN);

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::BadControlReturnError));
    }

    #[test]
    fn test_decode_bad_len() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::InvalidSsrmSizeError));
    }

    #[test]
    fn test_decode_bad_command() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            b'E',
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::BadCommandError));
    }

    #[test]
    fn test_decode_bad_message() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'P', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::BadSsrmMessageError));
    }
}
