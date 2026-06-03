//! Scénarios Session FSM + encode/decode STB — sans transport TCP.

use oftp_lib::{
    oeb::OftpExchangeBuffer,
    pdu::esid::EsidReason,
    pdu::ssid::Ssid,
    stb::StreamTransmissionBuffer,
    sth::{StreamTransmissionHeader, STH_SIZE},
    ProtocolError, ProtocolState, Session, SessionConfig, TransitionInput, TransitionOutput,
};

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

fn encode_wire(oeb: OftpExchangeBuffer) -> Vec<u8> {
    let mut stb = stb_with(oeb);
    let bytes = stb.encode().expect("encode STB");
    assert_eq!(bytes.len(), stb.header.length as usize);
    assert_eq!(
        stb.header.length,
        STH_SIZE + stb.oeb.encode().expect("encode OEB").len() as u32
    );
    bytes
}

fn decode_wire(bytes: &[u8]) -> OftpExchangeBuffer {
    let mut stb = StreamTransmissionBuffer::default();
    stb.decode(bytes).expect("decode STB");
    stb.oeb
}

fn wire_roundtrip(oeb: OftpExchangeBuffer) -> OftpExchangeBuffer {
    decode_wire(&encode_wire(oeb))
}

fn ssid_code(ssid: &Ssid) -> &str {
    std::str::from_utf8(&ssid.code).unwrap().trim_end()
}

/// Action utilisateur / locale → octets STB (ou `None` si pas d'émission).
fn emit(
    session: &mut Session,
    action: impl FnOnce(&mut Session) -> Result<TransitionOutput, ProtocolError>,
) -> Option<Vec<u8>> {
    let out = action(session).expect("transition");
    out.to_peer.map(encode_wire)
}

/// Octets STB reçus → transition pair → octets STB à renvoyer, le cas échéant.
fn deliver(session: &mut Session, wire: &[u8]) -> Option<Vec<u8>> {
    let oeb = decode_wire(wire);
    let out = session
        .transition(TransitionInput::Peer(oeb))
        .expect("transition peer");
    out.to_peer.map(encode_wire)
}

#[test]
fn session_handshake_and_release_via_stb() {
    let server_config = SessionConfig::default().with_code("SERVER01").unwrap();
    let client_config = SessionConfig::default().with_code("CLIENT01").unwrap();

    let mut server = Session::responder(server_config);
    let mut client = Session::initiator(client_config);

    // B — SSRM serveur → client
    let ssrm_wire = emit(&mut server, Session::accept_connection).expect("SSRM");
    assert!(matches!(
        decode_wire(&ssrm_wire),
        OftpExchangeBuffer::Ssrm(_)
    ));
    let client_ssid_wire = deliver(&mut client, &ssrm_wire).expect("SSID client");
    assert!(matches!(
        decode_wire(&client_ssid_wire),
        OftpExchangeBuffer::Ssid(_)
    ));
    assert_eq!(client.state, ProtocolState::InitWaitSsid);

    // E+G — SSID serveur → client
    let server_ssid_wire = deliver(&mut server, &client_ssid_wire).expect("SSID serveur");
    deliver(&mut client, &server_ssid_wire);

    assert_eq!(server.state, ProtocolState::IdleLi);
    assert_eq!(client.state, ProtocolState::IdleSp);
    assert_eq!(ssid_code(server.vars.peer_ssid.as_ref().unwrap()), "CLIENT01");
    assert_eq!(ssid_code(client.vars.peer_ssid.as_ref().unwrap()), "SERVER01");

    // Round-trip wire sur SSID serveur (idempotent)
    let ssid_oeb = decode_wire(&server_ssid_wire);
    assert_eq!(encode_wire(ssid_oeb.clone()), server_ssid_wire);
    assert_eq!(wire_roundtrip(ssid_oeb), decode_wire(&server_ssid_wire));

    // Fin de session — client ESID → serveur ESID → client Idle
    let client_esid_wire = emit(&mut client, Session::end_session).expect("ESID client");
    match decode_wire(&client_esid_wire) {
        OftpExchangeBuffer::Esid(esid) => {
            assert_eq!(esid.reason, EsidReason::Normal);
            assert!(esid.reason_text.is_empty());
        }
        other => panic!("attendu ESID, reçu {other:?}"),
    }
    assert_eq!(client.state, ProtocolState::WaitNDisc);

    let server_esid_wire = deliver(&mut server, &client_esid_wire).expect("ESID serveur");
    assert_eq!(server.state, ProtocolState::WaitNDisc);
    match decode_wire(&server_esid_wire) {
        OftpExchangeBuffer::Esid(esid) => assert_eq!(esid.reason, EsidReason::Normal),
        other => panic!("attendu ESID, reçu {other:?}"),
    }

    deliver(&mut client, &server_esid_wire);
    assert_eq!(client.state, ProtocolState::Idle);
    assert_eq!(server.state, ProtocolState::WaitNDisc);
}

#[test]
fn session_handshake_manual_confirm_via_stb() {
    let server_config = SessionConfig {
        auto_accept: false,
        ..SessionConfig::default().with_code("SERVER01").unwrap()
    };
    let client_config = SessionConfig::default().with_code("CLIENT01").unwrap();

    let mut server = Session::responder(server_config);
    let mut client = Session::initiator(client_config);

    let ssrm_wire = emit(&mut server, Session::accept_connection).unwrap();
    let client_ssid_wire = deliver(&mut client, &ssrm_wire).unwrap();

    // E — serveur attend validation app
    deliver(&mut server, &client_ssid_wire);
    assert_eq!(server.state, ProtocolState::RespWaitConRs);

    // G — confirm_partner → SSID serveur
    let server_ssid_wire = emit(&mut server, Session::confirm_partner).expect("SSID serveur");
    deliver(&mut client, &server_ssid_wire);
    assert_eq!(server.state, ProtocolState::IdleLi);
    assert_eq!(client.state, ProtocolState::IdleSp);
}

#[test]
fn session_esid_abort_handshake_via_stb() {
    let mut server = Session::responder(SessionConfig::default().with_code("SERVER01").unwrap());
    let mut client = Session::initiator(SessionConfig::default().with_code("CLIENT01").unwrap());

    let ssrm_wire = emit(&mut server, Session::accept_connection).unwrap();
    let client_ssid_wire = deliver(&mut client, &ssrm_wire).unwrap();
    deliver(&mut server, &client_ssid_wire);

    // Pair envoie ESID pendant InitWaitSsid (abort F)
    let abort_wire = encode_wire(OftpExchangeBuffer::Esid(oftp_lib::pdu::esid::Esid::normal()));
    deliver(&mut client, &abort_wire);
    assert_eq!(client.state, ProtocolState::Idle);
}

#[test]
fn session_peer_initiated_release_via_stb() {
    let mut server = Session::responder(SessionConfig::default().with_code("SERVER01").unwrap());
    let mut client = Session::initiator(SessionConfig::default().with_code("CLIENT01").unwrap());

    // Handshake rapide (auto_accept)
    let ssrm = emit(&mut server, Session::accept_connection).unwrap();
    let client_ssid = deliver(&mut client, &ssrm).unwrap();
    let server_ssid = deliver(&mut server, &client_ssid).unwrap();
    deliver(&mut client, &server_ssid);

    // Serveur initie la fermeture
    let esid_wire = emit(&mut server, Session::end_session).unwrap();
    assert_eq!(server.state, ProtocolState::WaitNDisc);

    let reply_wire = deliver(&mut client, &esid_wire).unwrap();
    assert_eq!(client.state, ProtocolState::WaitNDisc);

    deliver(&mut server, &reply_wire);
    assert_eq!(server.state, ProtocolState::Idle);
    assert_eq!(client.state, ProtocolState::WaitNDisc);
}

#[test]
fn session_full_lifecycle_wire_bytes_stable() {
    let mut server = Session::responder(SessionConfig::default().with_code("SERVER01").unwrap());
    let mut client = Session::initiator(SessionConfig::default().with_code("CLIENT01").unwrap());

    let ssrm = emit(&mut server, Session::accept_connection).unwrap();
    let client_ssid = deliver(&mut client, &ssrm).unwrap();
    let server_ssid = deliver(&mut server, &client_ssid).unwrap();
    deliver(&mut client, &server_ssid);

    let client_esid = emit(&mut client, Session::end_session).unwrap();
    let server_esid = deliver(&mut server, &client_esid).unwrap();
    deliver(&mut client, &server_esid);

    // Chaque PDU émis doit survivre à un aller-retour STB identique
    for wire in [ssrm, client_ssid, server_ssid, client_esid, server_esid] {
        let oeb = decode_wire(&wire);
        assert_eq!(encode_wire(oeb.clone()), wire);
        assert_eq!(wire_roundtrip(oeb), decode_wire(&wire));
    }
}
