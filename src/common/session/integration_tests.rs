//! Test d'intégration client ↔ serveur (handshake + transfert fichier).

use tokio::net::TcpListener;

use super::{ConnectOptions, OftpSession, Role};

#[tokio::test]
async fn handshake_and_file_transfer() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let temp_dir = std::env::temp_dir().join("oftp-rs-test");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let recv_dir = temp_dir.clone();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let options = ConnectOptions::default()
            .with_role(Role::Responder)
            .with_ssid_code("SERVERTESTORG00001")
            .unwrap();
        let mut session = OftpSession::new(options);
        session.accept(stream);
        session.run_handshake().await.unwrap();
        assert!(session.is_established());
        let path = session.run_receive_file(&recv_dir).await.unwrap();
        session.end_session().await.unwrap();
        path
    });

    let send_path = temp_dir.join("upload.bin");
    let payload = b"Bonjour OFTP2 POC\n";
    std::fs::write(&send_path, payload).unwrap();

    let options = ConnectOptions::default()
        .with_ssid_code("CLIENTTESTORG00001")
        .unwrap();
    let mut client = OftpSession::new(options);
    client.connect(&addr.to_string()).await.unwrap();
    client.run_handshake().await.unwrap();
    assert!(client.is_established());
    let sent = client.run_send_file(&send_path).await.unwrap();
    assert_eq!(sent, payload.len() as u64);
    client.end_session().await.unwrap();

    let received_path = server.await.unwrap();
    let received = std::fs::read(&received_path).unwrap();
    assert_eq!(received, payload);

    let _ = std::fs::remove_file(&send_path);
    let _ = std::fs::remove_file(&received_path);
}
