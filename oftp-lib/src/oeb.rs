use crate::pdu::ssrm::Ssrm;
use crate::pdu::ssid::Ssid;
use crate::pdu::ssrm::SsrmError;
use crate::pdu::ssid::SsidError;
use crate::pdu::ssrm::SSRMCMD;
use crate::pdu::ssid::SSIDCMD;

#[derive(Debug, Clone)]
pub enum OftpExchangeBuffer {
    None,
    Ssrm(Ssrm),
    Ssid(Ssid),
}

#[derive(Debug, thiserror::Error)]
pub enum OftpExchangeBufferError {
    #[error("SSRM error: {0}")]
    Ssrm(#[from] SsrmError),
    #[error("SSID error: {0}")]
    Ssid(#[from] SsidError),
    #[error("failed to decode OEB: Empty buffer !")]
    EmptyBufferError,
    #[error("failed to decode OEB: Invalid command !")]
    InvalidCommandError,
}

impl Default for OftpExchangeBuffer {
    fn default() -> Self {
        Self::None
    }
}

impl OftpExchangeBuffer {
    
    pub fn encode(&mut self) -> Result<Vec<u8>, OftpExchangeBufferError> {
        match self {
            Self::None => Ok(Vec::new()),
            Self::Ssrm(ssrm) => Ok(ssrm.encode()?),
            Self::Ssid(ssid) => Ok(ssid.encode()?),
        }
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<(), OftpExchangeBufferError> {

        if buf.is_empty() {
            return Err(OftpExchangeBufferError::EmptyBufferError);
        }

        match buf[0] {
            SSRMCMD => {
                let mut ssrm = Ssrm::default();
                ssrm.decode(buf)?;
                *self = Self::Ssrm(ssrm);
                Ok(())
            }
            SSIDCMD => {
                let mut ssid = Ssid::default();
                ssid.decode(buf)?;
                *self = Self::Ssid(ssid);
                Ok(())
            }
            _ => Err(OftpExchangeBufferError::InvalidCommandError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::ssrm::SSRM_LEN;
    use crate::pdu::ssid::SSID_LEN;

    fn ssrm_wire() -> [u8; SSRM_LEN] {
        [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D,
        ]
    }

    fn ssid_wire() -> Vec<u8> {
        Ssid::default()
            .encode()
            .expect("SSID sample encode")
    }

    #[test]
    fn decode_ssrm_from_none() {
        let mut oeb = OftpExchangeBuffer::None;
        let wire = ssrm_wire();

        oeb.decode(&wire).unwrap();

        match oeb {
            OftpExchangeBuffer::Ssrm(ssrm) => assert_eq!(ssrm.cr, 0x0D),
            other => panic!("expected Ssrm, got {other:?}"),
        }
    }

    #[test]
    fn decode_ssid_from_none() {
        let mut oeb = OftpExchangeBuffer::None;
        let wire = ssid_wire();

        oeb.decode(&wire).unwrap();

        match oeb {
            OftpExchangeBuffer::Ssid(ssid) => {
                assert_eq!(ssid.buffer_size, 2048);
                assert_eq!(ssid.cr, 0x0D);
            }
            other => panic!("expected Ssid, got {other:?}"),
        }
    }

    #[test]
    fn encode_ssrm() {
        let mut oeb = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
        let buf = oeb.encode().unwrap();

        assert_eq!(buf.len(), SSRM_LEN);
        assert_eq!(buf[0], SSRMCMD);
        assert_eq!(buf.as_slice(), &ssrm_wire());
    }

    #[test]
    fn encode_ssid() {
        let mut oeb = OftpExchangeBuffer::Ssid(Ssid::default());
        let buf = oeb.encode().unwrap();

        assert_eq!(buf.len(), SSID_LEN);
        assert_eq!(buf[0], SSIDCMD);
    }

    #[test]
    fn roundtrip_ssrm_decode_encode_decode() {
        let wire = ssrm_wire();
        let mut oeb = OftpExchangeBuffer::None;

        oeb.decode(&wire).unwrap();
        let encoded = oeb.encode().unwrap();
        assert_eq!(encoded.as_slice(), &wire);

        let mut oeb2 = OftpExchangeBuffer::None;
        oeb2.decode(&encoded).unwrap();
        match oeb2 {
            OftpExchangeBuffer::Ssrm(s) => assert_eq!(s.cr, 0x0D),
            other => panic!("expected Ssrm after second decode, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_ssid_decode_encode_decode() {
        let wire = ssid_wire();
        let mut oeb = OftpExchangeBuffer::None;

        oeb.decode(&wire).unwrap();
        let encoded = oeb.encode().unwrap();
        assert_eq!(encoded, wire);

        let mut oeb2 = OftpExchangeBuffer::None;
        oeb2.decode(&encoded).unwrap();
        assert!(matches!(oeb2, OftpExchangeBuffer::Ssid(_)));
    }

    #[test]
    fn decode_empty_buffer() {
        let mut oeb = OftpExchangeBuffer::None;
        let err = oeb.decode(&[]).unwrap_err();
        assert!(matches!(err, OftpExchangeBufferError::EmptyBufferError));
    }

    #[test]
    fn decode_invalid_command() {
        let mut oeb = OftpExchangeBuffer::None;
        let err = oeb.decode(&[b'Z']).unwrap_err();
        assert!(matches!(err, OftpExchangeBufferError::InvalidCommandError));
    }

    #[test]
    fn encode_none_is_empty() {
        let mut oeb = OftpExchangeBuffer::None;
        assert!(oeb.encode().unwrap().is_empty());
    }

    #[test]
    fn decode_replaces_previous_variant() {
        let mut oeb = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
        oeb.decode(&ssid_wire()).unwrap();
        assert!(matches!(oeb, OftpExchangeBuffer::Ssid(_)));
    }

    #[test]
    fn encode_ssrm_bad_cr_propagates() {
        let mut oeb = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0A });
        let err = oeb.encode().unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssrm(SsrmError::BadControlReturnError)
        ));
    }

    /// Échec decode : `*self` ne doit pas être assigné (reste `None`).
    #[test]
    fn decode_ssrm_wrong_length_leaves_none() {
        let mut oeb = OftpExchangeBuffer::None;
        let short = [SSRMCMD, b'X'];
        let err = oeb.decode(&short).unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssrm(SsrmError::InvalidSsrmSizeError)
        ));
        assert!(matches!(oeb, OftpExchangeBuffer::None));
    }

    /// `'X'` déclenche la branche SSID, pas `InvalidCommand`, même si le buffer est trop court.
    #[test]
    fn decode_ssid_wrong_length_is_ssid_error_not_invalid_command() {
        let mut oeb = OftpExchangeBuffer::None;
        let err = oeb.decode(&[SSIDCMD]).unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssid(SsidError::InvalidSsidSizeError)
        ));
        assert!(matches!(oeb, OftpExchangeBuffer::None));
    }

    #[test]
    fn decode_ssid_bad_protocol_level_via_oeb() {
        let mut wire = ssid_wire();
        wire[1] = b'9';
        let mut oeb = OftpExchangeBuffer::None;
        let err = oeb.decode(&wire).unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssid(SsidError::BadProtocolLevelError)
        ));
        assert!(matches!(oeb, OftpExchangeBuffer::None));
    }

    #[test]
    fn decode_ssrm_bad_message_via_oeb() {
        let mut wire = ssrm_wire();
        wire[1] = b'X'; // "XDETTE..." au lieu de "ODETTE..."
        let mut oeb = OftpExchangeBuffer::None;
        let err = oeb.decode(&wire).unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssrm(SsrmError::BadSsrmMessageError)
        ));
        assert!(matches!(oeb, OftpExchangeBuffer::None));
    }

    /// Buffer long avec commande SSRM : la taille SSRM (19) prime, pas un decode SSID.
    #[test]
    fn decode_ssrm_cmd_with_ssid_sized_buffer_fails_size_check() {
        let mut wire = vec![0u8; SSID_LEN];
        wire[0] = SSRMCMD;
        wire[1..SSRM_LEN].copy_from_slice(&ssrm_wire()[1..]);
        wire[SSRM_LEN - 1] = 0x0D;

        let mut oeb = OftpExchangeBuffer::None;
        let err = oeb.decode(&wire).unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssrm(SsrmError::InvalidSsrmSizeError)
        ));
    }

    /// Échec sur une branche SSID ne doit pas écraser un `Ssrm` déjà présent.
    #[test]
    fn failed_ssid_decode_does_not_clobber_existing_ssrm() {
        let mut oeb = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
        let mut bad = ssid_wire();
        bad[1] = b'9';

        let err = oeb.decode(&bad).unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssid(SsidError::BadProtocolLevelError)
        ));
        match oeb {
            OftpExchangeBuffer::Ssrm(s) => assert_eq!(s.cr, 0x0D),
            other => panic!("Ssrm must survive failed decode, got {other:?}"),
        }
    }

    #[test]
    fn invalid_command_leaves_existing_ssid_intact() {
        let original = Ssid::default();
        let mut oeb = OftpExchangeBuffer::Ssid(original.clone());

        let err = oeb.decode(&[b'Z', b'Y']).unwrap_err();
        assert!(matches!(err, OftpExchangeBufferError::InvalidCommandError));

        match &oeb {
            OftpExchangeBuffer::Ssid(s) => {
                assert_eq!(s.buffer_size, original.buffer_size);
                assert_eq!(s.code, original.code);
                assert_eq!(s.cr, original.cr);
            }
            other => panic!("expected Ssid unchanged, got {other:?}"),
        }
    }

    #[test]
    fn recover_after_failed_decode() {
        let mut oeb = OftpExchangeBuffer::None;
        assert!(oeb.decode(&[]).is_err());
        oeb.decode(&ssrm_wire()).unwrap();
        assert!(matches!(oeb, OftpExchangeBuffer::Ssrm(_)));
    }

    #[test]
    fn roundtrip_ssid_custom_fields_via_oeb() {
        let mut expected = Ssid::default();
        expected.code = *b"O00041234TESTORG000001234";
        expected.password = *b"SECRET  ";
        expected.buffer_size = 4096;
        expected.compression = true;
        expected.restart = true;
        expected.credit = 100;
        expected.auth = true;
        expected.user_data = *b"MYDATA  ";

        let mut wire = expected.encode().unwrap();
        wire[40] = b'S'; // SendOnly sur le fil
        wire[41] = b'Y';
        wire[42] = b'Y';

        let mut oeb = OftpExchangeBuffer::None;
        oeb.decode(&wire).unwrap();
        let back = oeb.encode().unwrap();
        assert_eq!(back, wire);

        let mut oeb2 = OftpExchangeBuffer::None;
        oeb2.decode(&back).unwrap();
        let OftpExchangeBuffer::Ssid(got) = oeb2 else {
            panic!("expected Ssid");
        };
        assert_eq!(got.code, expected.code);
        assert_eq!(got.password, expected.password);
        assert_eq!(got.buffer_size, expected.buffer_size);
        assert_eq!(got.mode.to_byte(), b'S');
        assert!(got.compression);
        assert!(got.restart);
        assert_eq!(got.credit, expected.credit);
        assert!(got.auth);
        assert_eq!(got.user_data, expected.user_data);
        assert_eq!(got.cr, expected.cr);
    }

    #[test]
    fn decode_ssrm_alternate_cr_8d() {
        let mut wire = ssrm_wire();
        wire[SSRM_LEN - 1] = 0x8D;
        let mut oeb = OftpExchangeBuffer::None;
        oeb.decode(&wire).unwrap();
        let encoded = oeb.encode().unwrap();
        assert_eq!(encoded[SSRM_LEN - 1], 0x8D);
    }

    #[test]
    fn encode_ssid_bad_credit_via_oeb() {
        let mut ssid = Ssid::default();
        ssid.credit = 1000;
        let mut oeb = OftpExchangeBuffer::Ssid(ssid);
        let err = oeb.encode().unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssid(SsidError::EncodeCreditError)
        ));
    }

    /// SSRM et SSID partagent la même première lettre en ASCII ? Non — mais `'I'`
    /// ne doit jamais être traité comme SSID : buffer SSID valide sauf octet 0 → erreur SSRM.
    #[test]
    fn ssid_wire_with_ssrm_command_byte_fails_at_ssrm_decode() {
        let mut wire = ssid_wire();
        wire[0] = SSRMCMD;
        let mut oeb = OftpExchangeBuffer::None;
        let err = oeb.decode(&wire).unwrap_err();
        assert!(matches!(
            err,
            OftpExchangeBufferError::Ssrm(SsrmError::InvalidSsrmSizeError)
        ));
    }
}