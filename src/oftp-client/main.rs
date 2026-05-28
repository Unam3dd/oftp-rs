use std::path::PathBuf;

use clap::Parser;
use oftp_rs::{ConnectOptions, OftpSession, SsidFieldError};

#[derive(Parser)]
#[command(about = "Client OFTP2 (handshake initiateur + envoi fichier)")]
struct Args {
    /// Adresse du serveur (ex. 127.0.0.1:3305).
    target: String,

    /// Fichier local à envoyer après le handshake.
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,

    /// Code identifiant ODETTE (SSIDCODE, 25 caractères max).
    #[arg(long, value_name = "CODE", default_value = "ODSTALES")]
    ssid_code: String,

    /// Mot de passe ODETTE (SSIDPSWD, 8 caractères max).
    #[arg(long, value_name = "PASSWORD")]
    password: Option<String>,
}

fn build_options(args: &Args) -> Result<ConnectOptions, SsidFieldError> {
    let mut options = ConnectOptions::default().with_ssid_code(&args.ssid_code)?;
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

    let mut session = OftpSession::new(options);

    if let Err(err) = session.connect(&args.target).await {
        eprintln!("Connexion à {} impossible : {err}", args.target);
        std::process::exit(1);
    }

    println!(
        "TCP connecté à {} — {} (état RFC: {})",
        args.target,
        session.phase(),
        session.state()
    );

    match session.run_handshake().await {
        Ok(()) if session.is_established() => {
            println!(
                "Handshake OK — {} (état RFC: {})",
                session.phase(),
                session.state()
            );
            if args.file.is_some() {
                println!(
                    "Avant l'envoi : lancer oftp-server sur le port de rappel, ex.\n  \
                     cargo run --bin oftp-server -- --listen 0.0.0.0:3306 --ssid-code \"{}\"",
                    args.ssid_code
                );
            }
        }
        Ok(()) => {
            eprintln!(
                "Handshake terminé mais état inattendu : {} ({})",
                session.state(),
                session.phase()
            );
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("Handshake : {err}");
            std::process::exit(1);
        }
    }

    if let Some(path) = &args.file {
        let meta = std::fs::metadata(path);
        match meta {
            Ok(m) => println!(
                "Fichier local : {} ({} octets sur disque)",
                path.display(),
                m.len()
            ),
            Err(err) => {
                eprintln!("Fichier introuvable {} : {err}", path.display());
                std::process::exit(1);
            }
        }

        match session.run_send_file(path).await {
            Ok(bytes) => {
                println!(
                    "Fichier envoyé sur le fil OFTP : {} ({} octets DATA)",
                    path.display(),
                    bytes
                );
                if bytes == 0 {
                    eprintln!("Attention : 0 octet DATA — mendelson affichera aussi 0 octet reçu");
                }
            }
            Err(err) => {
                eprintln!("Envoi fichier : {err}");
                std::process::exit(1);
            }
        }
    }

    if let Err(err) = session.end_session().await {
        eprintln!("Fin de session : {err}");
        std::process::exit(1);
    }

    println!(
        "Déconnecté de {} — {} (état RFC: {})",
        args.target,
        session.phase(),
        session.state()
    );
}
