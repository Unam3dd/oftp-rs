//! Scénario handshake §4.2 — SSRM, SSID ×2, puis ESID ×2 (fin de session), en mémoire.

use oftp_lib::oeb::OftpExchangeBuffer;
use oftp_lib::pdu::esid::{Esid, ESID_REASON_NORMAL};
use oftp_lib::pdu::ssrm::Ssrm;
use oftp_lib::pdu::ssid::Ssid;
use oftp_lib::sth::{StreamTransmissionHeader, STH_SIZE};
use oftp_lib::stb::StreamTransmissionBuffer;

fn stb_with(oeb: OftpExchangeBuffer) -> StreamTransmissionBuffer {
    StreamTransmissionBuffer {
        header: StreamTransmissionHeader {
            version: 1,
            flags: 0,
            length: 0,
        },
        oeb,
    }
}

fn encode(wire: &mut StreamTransmissionBuffer) -> Vec<u8> {
    let bytes = wire.encode().expect("encode STB");
    assert_eq!(bytes.len(), wire.header.length as usize);
    assert_eq!(
        wire.header.length,
        STH_SIZE + wire.oeb.encode().expect("encode OEB").len() as u32
    );
    bytes
}

fn decode(bytes: &[u8]) -> StreamTransmissionBuffer {
    let mut stb = StreamTransmissionBuffer::default();
    stb.decode(bytes).expect("decode STB");
    stb
}

fn ssid_with_code(code: &str) -> Ssid {
    let mut ssid = Ssid::default();
    ssid.set_code(code).unwrap();
    ssid
}

/// Séquence RFC : Responder → SSRM, Initiator → SSID, Responder → SSID,
/// puis fermeture : Initiator → ESID, Responder → ESID.
#[test]
fn handshake_scenario_ssrm_then_two_ssid() {
    // --- 1. Serveur (Responder) envoie SSRM ---
    let mut server_ssrm = stb_with(OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D }));
    let wire_ssrm = encode(&mut server_ssrm);

    // --- 2. Client (Initiator) reçoit SSRM ---
    let client_rx = decode(&wire_ssrm);
    match client_rx.oeb {
        OftpExchangeBuffer::Ssrm(ssrm) => assert_eq!(ssrm.cr, 0x0D),
        other => panic!("attendu SSRM, reçu {other:?}"),
    }

    // --- 3. Client envoie SSID (Initiator) ---
    let mut client_ssid = stb_with(OftpExchangeBuffer::Ssid(ssid_with_code("CLIENT01")));
    let wire_ssid_client = encode(&mut client_ssid);

    // --- 4. Serveur reçoit SSID client ---
    let server_rx_client = decode(&wire_ssid_client);
    match &server_rx_client.oeb {
        OftpExchangeBuffer::Ssid(ssid) => {
            let code = std::str::from_utf8(&ssid.code).unwrap().trim_end();
            assert_eq!(code, "CLIENT01");
        }
        other => panic!("attendu SSID, reçu {other:?}"),
    }

    // --- 5. Serveur envoie SSID (Responder) ---
    let mut server_ssid = stb_with(OftpExchangeBuffer::Ssid(ssid_with_code("SERVER01")));
    let wire_ssid_server = encode(&mut server_ssid);

    // --- 6. Client reçoit SSID serveur ---
    let client_rx_server = decode(&wire_ssid_server);
    match &client_rx_server.oeb {
        OftpExchangeBuffer::Ssid(ssid) => {
            let code = std::str::from_utf8(&ssid.code).unwrap().trim_end();
            assert_eq!(code, "SERVER01");
        }
        other => panic!("attendu SSID, reçu {other:?}"),
    }

    // --- 7. Re-encodage idempotent (bytes stables) ---
    let mut roundtrip = client_rx_server;
    let again = encode(&mut roundtrip);
    assert_eq!(again, wire_ssid_server);

    // --- 8. Client envoie ESID (fin de session normale, ESIDREAS = 00) ---
    let mut client_esid = stb_with(OftpExchangeBuffer::Esid(Esid::normal()));
    let wire_esid_client = encode(&mut client_esid);

    // --- 9. Serveur reçoit ESID client ---
    let server_rx_esid = decode(&wire_esid_client);
    match &server_rx_esid.oeb {
        OftpExchangeBuffer::Esid(esid) => {
            assert_eq!(esid.reason, ESID_REASON_NORMAL);
            assert!(esid.reason_text.is_empty());
            assert_eq!(esid.cr, 0x0D);
        }
        other => panic!("attendu ESID, reçu {other:?}"),
    }

    // --- 10. Serveur répond ESID ---
    let mut server_esid = stb_with(OftpExchangeBuffer::Esid(Esid::normal()));
    let wire_esid_server = encode(&mut server_esid);

    // --- 11. Client reçoit ESID serveur ---
    let client_rx_esid = decode(&wire_esid_server);
    match &client_rx_esid.oeb {
        OftpExchangeBuffer::Esid(esid) => {
            assert_eq!(esid.reason, ESID_REASON_NORMAL);
            assert!(esid.reason_text.is_empty());
            assert_eq!(esid.cr, 0x0D);
        }
        other => panic!("attendu ESID, reçu {other:?}"),
    }

    // --- 12. Re-encodage idempotent ESID ---
    let mut esid_roundtrip = client_rx_esid;
    let esid_again = encode(&mut esid_roundtrip);
    assert_eq!(esid_again, wire_esid_server);
}
