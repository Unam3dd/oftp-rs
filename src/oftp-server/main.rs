use std::path::PathBuf;

use clap::Parser;
use oftp_rs::{ConnectOptions, OftpSession, Role, SsidFieldError};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Parser)]
#[command(about = "Serveur OFTP2 (réception fichier + rappel EERP mendelson)")]
struct Args {
    /// Adresse d'écoute (ex. 127.0.0.1:3306 pour rappel EERP mendelson).
    #[arg(default_value = "127.0.0.1:3306")]
    listen: String,

    /// Répertoire où enregistrer les fichiers reçus.
    #[arg(long, value_name = "DIR", default_value = ".")]
    output_dir: PathBuf,

    /// Code identifiant ODETTE (SSIDCODE, 25 caractères max).
    #[arg(long, value_name = "CODE", default_value = "ODSTALES")]
    ssid_code: String,

    /// Mot de passe ODETTE (SSIDPSWD, 8 caractères max).
    #[arg(long, value_name = "PASSWORD")]
    password: Option<String>,
}

fn build_options(args: &Args) -> Result<ConnectOptions, SsidFieldError> {
    let mut options = ConnectOptions::default()
        .with_role(Role::Responder)
        .with_ssid_code(&args.ssid_code)?;
    if let Some(password) = &args.password {
        options = options.with_password(password)?;
    }
    Ok(options)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "oftp_rs=debug,info".into()),
        )
        .init();

    let args = Args::parse();

    let options = match build_options(&args) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("Configuration SSID : {err}");
            std::process::exit(1);
        }
    };

    let listener = match TcpListener::bind(&args.listen).await {
        Ok(l) => l,
        Err(err) => {
            eprintln!("Écoute sur {} impossible : {err}", args.listen);
            std::process::exit(1);
        }
    };

    println!("Serveur OFTP — écoute sur {}", args.listen);
    println!("  • Réception fichier (SFID) ou rappel EERP mendelson");
    println!("  • Avec mendelson sur :3305, configurer le partenaire ODSTALES vers ce port pour l'appel sortant");
    println!("Fichiers reçus dans : {}", args.output_dir.display());

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(pair) => pair,
            Err(err) => {
                eprintln!("accept : {err}");
                continue;
            }
        };

        info!(%addr, "connexion entrante");
        let mut session = OftpSession::new(options.clone());
        session.accept(stream);

        if let Err(err) = session.run_handshake().await {
            eprintln!("Handshake avec {addr} : {err}");
            let _ = session.close().await;
            continue;
        }

        println!(
            "Handshake OK avec {addr} — {} (état RFC: {})",
            session.phase(),
            session.state()
        );

        match session.run_responder_service(&args.output_dir).await {
            Ok(()) => println!("Service terminé avec {addr}"),
            Err(err) => eprintln!("Session {addr} : {err}"),
        }

        if let Err(err) = session.end_session().await {
            eprintln!("Fin de session : {err}");
        }
    }
}
