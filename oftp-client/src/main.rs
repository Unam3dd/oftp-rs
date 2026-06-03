use oftp_lib::oeb::OftpExchangeBuffer;
use oftp_lib::{read_stb_async, write_stb_async, ProtocolState, Session, SessionConfig, TransitionInput};
use tokio::net::TcpStream;
use oftp_lib::codec::pdu::ssid::protocol_level::ProtocolLevel;

use oftp_lib::codec::pdu::secd::Secd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3305".to_string());

    let mut stream = TcpStream::connect(&addr).await?;
    println!("Connecté à {addr} (local {:?})", stream.local_addr()?);

    let config = SessionConfig::default().with_level(ProtocolLevel::Rev20).with_authentication(true).with_code("O1999MENDESTALES")?;

    let mut session = Session::initiator(config);
    assert_eq!(session.state, ProtocolState::InitWaitRm);

    // 1. Le serveur envoie SSRM (transition B côté responder)
    let stb = read_stb_async(&mut stream).await?;
    match &stb.oeb {
        OftpExchangeBuffer::Ssrm(ssrm) => {
            println!("{ssrm}");
        }
        other => {
            return Err(format!("attendu SSRM en premier, reçu {other:?}").into());
        }
    }

    // 2. FSM : H — répondre SSID
    let out = session.transition(TransitionInput::Peer(stb.oeb))?;
    println!("État après SSRM: {:?}", session.state);

    if let Some(oeb) = out.to_peer {
        if let OftpExchangeBuffer::Ssid(ssid) = &oeb {
            println!("{ssid}");
        }
        write_stb_async(&mut stream, oeb).await?;
        println!("SSID client envoyé");
    }

    // 3. Attendre SSID serveur (transition D)
    let stb = read_stb_async(&mut stream).await?;
    match &stb.oeb {
        OftpExchangeBuffer::Ssid(ssid) => {
            println!("{ssid}");
        }
        OftpExchangeBuffer::Esid(esid) => {
            return Err(format!("ESID reçu {}", esid.reason.to_string()).into());
        }
        other => {
            return Err(format!("Erreur PDU Inattendu, reçu {other:?}").into());
        }
    }

    let out = session.transition(TransitionInput::Peer(stb.oeb))?;
    println!("Handshake OK — état {:?}", session.state);

    if let Some(oeb) = out.to_peer {
        if let OftpExchangeBuffer::Secd(secd) = &oeb {
            println!("{secd}");
        }
        write_stb_async(&mut stream, oeb).await?;
        println!("SECD envoyé");
    }

    let stb = read_stb_async(&mut stream).await?;
    match &stb.oeb {
        OftpExchangeBuffer::Auch(auch) => {
            println!("{auch}");
        }
        other => {
            return Err(format!("Erreur PDU Inattendu, reçu {other:?}").into());
        }
    }

    let out = session.transition(TransitionInput::Peer(stb.oeb))?;
    println!("État après AUCH: {:?}", session.state);  // → WfSecd (initiator)

    if let Some(oeb) = out.to_peer {
        if let OftpExchangeBuffer::Aurp(aurp) = &oeb {
            println!("{aurp}");
        }
        write_stb_async(&mut stream, oeb).await?;
        println!("AURP envoyé");
    }

    let stb = read_stb_async(&mut stream).await?;
    match &stb.oeb {
        OftpExchangeBuffer::Auch(auch) => {
            println!("{auch}");
        }
        other => {
            return Err(format!("Erreur PDU Inattendu, reçu {other:?}").into());
        }
    }

    let out = session.end_session()?;

    if let Some(oeb) = out.to_peer {
        if let OftpExchangeBuffer::Esid(esid) = &oeb {
            println!("{esid}");
        }
        write_stb_async(&mut stream, oeb).await?;
        println!("ESID client envoyé");
    }

    println!("Session terminée");

    Ok(())
}
