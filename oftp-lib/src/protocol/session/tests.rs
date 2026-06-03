use super::super::config::{SessionConfig, SessionRole};
use super::super::error::ProtocolError;
use super::super::input::TransitionInput;
use super::super::negotiate::PartnerPolicy;
use super::super::state::ProtocolState;
use super::Session;
use crate::codec::oeb::OftpExchangeBuffer;
use crate::codec::pdu::esid::{Esid, EsidReason};
use crate::codec::pdu::ssrm::Ssrm;
use crate::codec::pdu::ssid::{Ssid, SsidMode};

fn ssid_code(ssid: &Ssid) -> &str {
    std::str::from_utf8(&ssid.code).unwrap().trim_end()
}

#[test]
fn initiator_starts_in_init_wait_rm() {
    let session = Session::initiator(SessionConfig::default());
    assert_eq!(session.state, ProtocolState::InitWaitRm);
    assert_eq!(session.role, SessionRole::Initiator);
    assert!(session.vars.caller);
}

#[test]
fn responder_starts_idle() {
    let session = Session::responder(SessionConfig::default());
    assert_eq!(session.state, ProtocolState::Idle);
    assert_eq!(session.role, SessionRole::Responder);
    assert!(!session.vars.caller);
}

#[test]
fn config_with_code() {
    let config = SessionConfig::default().with_code("CLIENT01").unwrap();
    assert_eq!(ssid_code(&config.local_ssid), "CLIENT01");
}

#[test]
fn transition_b_send_ssrm() {
    let mut server = Session::responder(SessionConfig::default());

    let out = server.accept_connection().unwrap();

    assert_eq!(server.state, ProtocolState::RespNcOnly);
    assert_eq!(out.next, ProtocolState::RespNcOnly);
    assert!(matches!(out.to_peer, Some(OftpExchangeBuffer::Ssrm(_))));
}

#[test]
fn accept_connection_rejects_initiator() {
    let mut client = Session::initiator(SessionConfig::default());
    assert!(matches!(
        client.accept_connection().unwrap_err(),
        ProtocolError::InvalidTransition {
            state: ProtocolState::InitWaitRm,
            event: "N_CON_IND",
        }
    ));
}

#[test]
fn handshake_fsm_ssrm_and_two_ssid() {
    let server_config = SessionConfig::default().with_code("SERVER01").unwrap();
    let client_config = SessionConfig::default().with_code("CLIENT01").unwrap();

    let mut server = Session::responder(server_config);
    let mut client = Session::initiator(client_config);

    // B — serveur envoie SSRM
    let out_b = server.accept_connection().unwrap();
    let ssrm = out_b.to_peer.expect("SSRM");

    // H — client envoie SSID
    let out_h = client
        .transition(TransitionInput::Peer(ssrm))
        .expect("transition H");
    assert_eq!(out_h.next, ProtocolState::InitWaitSsid);
    let client_ssid = out_h.to_peer.expect("SSID client");

    // E+G — serveur répond SSID
    let out_g = server
        .transition(TransitionInput::Peer(client_ssid))
        .expect("transition E+G");
    assert_eq!(server.state, ProtocolState::IdleLi);
    assert_eq!(out_g.next, ProtocolState::IdleLi);
    let server_ssid = out_g.to_peer.expect("SSID serveur");

    // D — client reçoit SSID serveur
    let out_d = client
        .transition(TransitionInput::Peer(server_ssid))
        .expect("transition D");
    assert_eq!(client.state, ProtocolState::IdleSp);
    assert_eq!(out_d.next, ProtocolState::IdleSp);
    assert!(out_d.to_peer.is_none());

    assert_eq!(ssid_code(server.vars.peer_ssid.as_ref().unwrap()), "CLIENT01");
    assert_eq!(ssid_code(client.vars.peer_ssid.as_ref().unwrap()), "SERVER01");
    assert_eq!(server.vars.window, 50);
    assert_eq!(client.vars.buf_size, 2048);
    assert_eq!(client.vars.mode, SsidMode::Both);
}

#[test]
fn initiator_rejects_incompatible_ssid_with_esid() {
    let mut client = Session::initiator(SessionConfig::default().with_code("CLIENT01").unwrap());
    let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
    client.transition(TransitionInput::Peer(ssrm)).unwrap();
    assert_eq!(client.state, ProtocolState::InitWaitSsid);

    let mut bad_peer = Ssid::default();
    bad_peer.mode = SsidMode::SendOnly;
    client.config.local_ssid.mode = SsidMode::SendOnly;

    let out = client
        .transition(TransitionInput::Peer(OftpExchangeBuffer::Ssid(bad_peer)))
        .unwrap();

    assert_eq!(client.state, ProtocolState::WaitNDisc);
    match out.to_peer {
        Some(OftpExchangeBuffer::Esid(esid)) => {
            assert_eq!(esid.reason, EsidReason::ModeOrCapabilitiesIncompatible);
        }
        other => panic!("attendu ESID, reçu {other:?}"),
    }
}

#[test]
fn responder_without_auto_accept_waits_after_peer_ssid() {
    let config = SessionConfig {
        auto_accept: false,
        ..SessionConfig::default().with_code("SERVER01").unwrap()
    };
    let mut server = Session::responder(config);
    server.accept_connection().unwrap();

    let mut client = Session::initiator(SessionConfig::default().with_code("CLIENT01").unwrap());
    let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
    let client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();

    let out = server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap();

    assert_eq!(server.state, ProtocolState::RespWaitConRs);
    assert_eq!(out.next, ProtocolState::RespWaitConRs);
    assert!(out.to_peer.is_none());
}

#[test]
fn invalid_transition_wrong_pdu() {
    let mut client = Session::initiator(SessionConfig::default());
    let err = client
        .transition(TransitionInput::Peer(OftpExchangeBuffer::Ssid(Ssid::default())))
        .unwrap_err();
    assert!(matches!(
        err,
        ProtocolError::InvalidTransition {
            state: ProtocolState::InitWaitRm,
            ..
        }
    ));
}

#[test]
fn transition_g_after_e_without_auto_accept() {
    let config = SessionConfig {
        auto_accept: false,
        ..SessionConfig::default().with_code("SERVER01").unwrap()
    };
    let mut server = Session::responder(config);
    server.accept_connection().unwrap();

    let mut client = Session::initiator(SessionConfig::default().with_code("CLIENT01").unwrap());
    let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
    let client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();

    server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap();
    assert_eq!(server.state, ProtocolState::RespWaitConRs);

    let out = server.confirm_partner().unwrap();
    assert_eq!(server.state, ProtocolState::IdleLi);
    assert_eq!(out.next, ProtocolState::IdleLi);
    assert!(matches!(out.to_peer, Some(OftpExchangeBuffer::Ssid(_))));
}

#[test]
fn confirm_partner_rejects_initiator() {
    let mut client = Session::initiator(SessionConfig::default());
    assert!(matches!(
        client.confirm_partner().unwrap_err(),
        ProtocolError::InvalidTransition {
            state: ProtocolState::InitWaitRm,
            event: "F_CONNECT_RS",
        }
    ));
}

#[test]
fn end_session_esid_round_trip() {
    let mut client = Session::initiator(SessionConfig::default());
    client.state = ProtocolState::IdleSp;

    let out = client.end_session().unwrap();
    assert_eq!(client.state, ProtocolState::WaitNDisc);
    assert_eq!(out.next, ProtocolState::WaitNDisc);
    assert!(matches!(out.to_peer, Some(OftpExchangeBuffer::Esid(_))));

    let peer_esid = OftpExchangeBuffer::Esid(Esid::normal());
    let out2 = client
        .transition(TransitionInput::Peer(peer_esid))
        .unwrap();
    assert_eq!(client.state, ProtocolState::Idle);
    assert_eq!(out2.next, ProtocolState::Idle);
    assert!(out2.to_peer.is_none());
}

#[test]
fn peer_esid_in_idle_sp_replies_and_waits_ndisc() {
    let mut server = Session::responder(SessionConfig::default());
    server.state = ProtocolState::IdleLi;

    let out = server
        .transition(TransitionInput::Peer(OftpExchangeBuffer::Esid(Esid::normal())))
        .unwrap();

    assert_eq!(server.state, ProtocolState::WaitNDisc);
    assert_eq!(out.next, ProtocolState::WaitNDisc);
    assert!(matches!(out.to_peer, Some(OftpExchangeBuffer::Esid(_))));
}

#[test]
fn esid_during_handshake_aborts_to_idle() {
    let mut client = Session::initiator(SessionConfig::default());
    let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
    client.transition(TransitionInput::Peer(ssrm)).unwrap();
    assert_eq!(client.state, ProtocolState::InitWaitSsid);

    let out = client
        .transition(TransitionInput::Peer(OftpExchangeBuffer::Esid(Esid::normal())))
        .unwrap();

    assert_eq!(client.state, ProtocolState::Idle);
    assert_eq!(out.next, ProtocolState::Idle);
    assert!(out.to_peer.is_none());
}

#[test]
fn end_session_rejects_wrong_state() {
    let mut client = Session::initiator(SessionConfig::default());
    assert!(matches!(
        client.end_session().unwrap_err(),
        ProtocolError::InvalidTransition {
            state: ProtocolState::InitWaitRm,
            event: "F_RELEASE_RQ",
        }
    ));
}

#[test]
fn handshake_send_only_and_receive_only_negotiates_both() {
    let server_config = SessionConfig::default()
        .with_code("SERVER01")
        .unwrap()
        .with_mode(SsidMode::ReceiveOnly);
    let client_config = SessionConfig::default()
        .with_code("CLIENT01")
        .unwrap()
        .with_mode(SsidMode::SendOnly);

    let mut server = Session::responder(server_config);
    let mut client = Session::initiator(client_config);

    let ssrm = server.accept_connection().unwrap().to_peer.unwrap();
    let client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();
    let server_ssid = server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap()
        .to_peer
        .unwrap();
    client
        .transition(TransitionInput::Peer(server_ssid))
        .unwrap();

    assert_eq!(client.vars.mode, SsidMode::Both);
    assert_eq!(server.vars.mode, SsidMode::Both);
}

#[test]
fn handshake_merges_compression_when_both_enabled() {
    let server_config = SessionConfig::default()
        .with_code("SERVER01")
        .unwrap()
        .with_compression(true);
    let client_config = SessionConfig::default()
        .with_code("CLIENT01")
        .unwrap()
        .with_compression(true);

    let (client, server) = complete_handshake(
        Session::initiator(client_config),
        Session::responder(server_config),
    );
    assert!(client.vars.compression);
    assert!(server.vars.compression);
}

#[test]
fn handshake_compression_off_if_peer_disabled() {
    let server_config = SessionConfig::default()
        .with_code("SERVER01")
        .unwrap()
        .with_compression(true);
    let client_config = SessionConfig::default()
        .with_code("CLIENT01")
        .unwrap()
        .with_compression(false);

    let (client, _) = complete_handshake(
        Session::initiator(client_config),
        Session::responder(server_config),
    );
    assert!(!client.vars.compression);
}

#[test]
fn initiator_rejects_unknown_server_code_via_partner_policy() {
    let config = SessionConfig::default()
        .with_code("CLIENT01")
        .unwrap()
        .with_partner_policy(PartnerPolicy::RequireCode("TRUSTED_SERVER".into()));
    let mut client = Session::initiator(config);
    let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
    client.transition(TransitionInput::Peer(ssrm)).unwrap();

    let mut peer = Ssid::default();
    peer.set_code("WRONG_SERVER").unwrap();
    let out = client
        .transition(TransitionInput::Peer(OftpExchangeBuffer::Ssid(peer)))
        .unwrap();

    assert_eq!(client.state, ProtocolState::WaitNDisc);
    match out.to_peer {
        Some(OftpExchangeBuffer::Esid(esid)) => {
            assert_eq!(esid.reason, EsidReason::UserCodeNotKnown);
        }
        other => panic!("attendu ESID, reçu {other:?}"),
    }
}

#[test]
fn responder_cap_mode_rejects_peer_without_esid() {
    let config = SessionConfig::default()
        .with_code("SERVER01")
        .unwrap()
        .with_cap_mode(SsidMode::SendOnly);
    let mut server = Session::responder(config);
    server.accept_connection().unwrap();

    let mut client = Session::initiator(SessionConfig::default().with_code("C").unwrap());
    let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
    let mut client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();
    if let OftpExchangeBuffer::Ssid(ref mut s) = client_ssid {
        s.set_mode(SsidMode::SendOnly);
    }

    let out = server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap();

    assert_eq!(server.state, ProtocolState::Idle);
    assert_eq!(out.next, ProtocolState::Idle);
    assert!(out.to_peer.is_none());
}

fn auth_config(code: &str) -> SessionConfig {
    SessionConfig::default()
        .with_code(code)
        .unwrap()
        .with_authentication(true)
        .with_auth_plaintext_stub(true)
}

#[test]
fn handshake_with_auth_reaches_idle_sp_and_idle_li() {
    let server_config = auth_config("SERVER01");
    let client_config = auth_config("CLIENT01");

    let mut server = Session::responder(server_config);
    let mut client = Session::initiator(client_config);

    let ssrm = server.accept_connection().unwrap().to_peer.unwrap();
    let client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();
    let server_ssid = server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap()
        .to_peer
        .unwrap();

    let out_d = client
        .transition(TransitionInput::Peer(server_ssid))
        .unwrap();
    assert_eq!(out_d.next, ProtocolState::WfAuch);
    assert!(matches!(out_d.to_peer, Some(OftpExchangeBuffer::Secd(_))));

    let secd = out_d.to_peer.unwrap();
    let out_j = server.transition(TransitionInput::Peer(secd)).unwrap();
    assert_eq!(out_j.next, ProtocolState::WfAurp);
    let auch = out_j.to_peer.expect("AUCH");

    let out_i = client.transition(TransitionInput::Peer(auch)).unwrap();
    assert_eq!(out_i.next, ProtocolState::WfSecd);
    let aurp = out_i.to_peer.expect("AURP");

    let out_k1 = server.transition(TransitionInput::Peer(aurp)).unwrap();
    assert_eq!(out_k1.next, ProtocolState::WfAuch);
    let secd2 = out_k1.to_peer.expect("SECD tour 2");

    let out_j2 = client.transition(TransitionInput::Peer(secd2)).unwrap();
    let auch2 = out_j2.to_peer.expect("AUCH tour 2");

    let out_i2 = server.transition(TransitionInput::Peer(auch2)).unwrap();
    assert_eq!(server.state, ProtocolState::IdleLi);
    let aurp2 = out_i2.to_peer.expect("AURP tour 2");

    let out_k2 = client.transition(TransitionInput::Peer(aurp2)).unwrap();
    assert_eq!(client.state, ProtocolState::IdleSp);
    assert!(out_k2.to_peer.is_none());
    assert!(client.vars.authentication);
}

#[test]
fn auth_protocol_violation_emits_esid_02() {
    let mut client = Session::initiator(auth_config("CLIENT01"));
    let mut server = Session::responder(auth_config("SERVER01"));

    let ssrm = server.accept_connection().unwrap().to_peer.unwrap();
    let client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();
    let server_ssid = server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap()
        .to_peer
        .unwrap();
    client
        .transition(TransitionInput::Peer(server_ssid))
        .unwrap();

    assert_eq!(client.state, ProtocolState::WfAuch);

    let out = client
        .transition(TransitionInput::Peer(OftpExchangeBuffer::Ssid(Ssid::default())))
        .unwrap();

    assert_eq!(client.state, ProtocolState::WaitNDisc);
    match out.to_peer {
        Some(OftpExchangeBuffer::Esid(esid)) => {
            assert_eq!(esid.reason, EsidReason::ProtocolViolation);
        }
        other => panic!("attendu ESID(02), reçu {other:?}"),
    }
}

#[test]
fn auth_invalid_aurp_emits_esid_11() {
    let mut server = Session::responder(auth_config("SERVER01"));
    let mut client = Session::initiator(auth_config("CLIENT01"));

    let ssrm = server.accept_connection().unwrap().to_peer.unwrap();
    let client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();
    let server_ssid = server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap()
        .to_peer
        .unwrap();
    let secd = client
        .transition(TransitionInput::Peer(server_ssid))
        .unwrap()
        .to_peer
        .unwrap();
    let auch = server
        .transition(TransitionInput::Peer(secd))
        .unwrap()
        .to_peer
        .unwrap();
    client.transition(TransitionInput::Peer(auch)).unwrap();

    let bad_aurp = OftpExchangeBuffer::Aurp(crate::codec::pdu::aurp::Aurp {
        response: [0xFFu8; 20],
    });
    let out = server.transition(TransitionInput::Peer(bad_aurp)).unwrap();

    assert_eq!(server.state, ProtocolState::WaitNDisc);
    match out.to_peer {
        Some(OftpExchangeBuffer::Esid(esid)) => {
            assert_eq!(esid.reason, EsidReason::InvalidChallengeResponse);
        }
        other => panic!("attendu ESID(11), reçu {other:?}"),
    }
}

/// Helpers handshake complet → client en `IdleSp`.
fn complete_handshake(
    mut client: Session,
    mut server: Session,
) -> (Session, Session) {
    let ssrm = server.accept_connection().unwrap().to_peer.unwrap();
    let client_ssid = client
        .transition(TransitionInput::Peer(ssrm))
        .unwrap()
        .to_peer
        .unwrap();
    let server_ssid = server
        .transition(TransitionInput::Peer(client_ssid))
        .unwrap()
        .to_peer
        .unwrap();
    client
        .transition(TransitionInput::Peer(server_ssid))
        .unwrap();
    (client, server)
}
