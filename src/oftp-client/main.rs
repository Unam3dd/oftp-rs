use clap::Parser;
use oftp_rs::{ConnectOptions, OftpSession, SsidFieldError};

#[derive(Parser)]
#[command(about = "Client OFTP2 (handshake initiateur)")]
struct Args {
    /// Adresse du serveur (ex. 127.0.0.1:3305).
    target: String,

    /// Code identifiant ODETTE (SSIDCODE, 25 caractères max).
    #[arg(long, value_name = "CODE", default_value = "O01779122072341")]
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
    println!("SSIDCODE local : {}", args.ssid_code);

    if let Some(stream) = session.stream() {
        match stream.peer_addr() {
            Ok(addr) => println!("Pair : {addr}"),
            Err(err) => eprintln!("peer_addr : {err}"),
        }
    }

    match session.run_handshake().await {
        Ok(()) if session.is_established() => {
            println!(
                "Handshake OK — {} (état RFC: {}, prêt pour transfert fichier)",
                session.phase(),
                session.state()
            );
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

    if let Err(err) = session.close().await {
        eprintln!("Fermeture : {err}");
    }

    println!(
        "Déconnecté de {} — {} (état RFC: {})",
        args.target,
        session.phase(),
        session.state()
    );
}
