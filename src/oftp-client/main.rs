use clap::Parser;
use oftp_rs::stream::StreamTransmissionHeader;
use oftp_rs::commands::ssrm::SSRMMSG;
use std::net::{Shutdown, TcpStream};


#[derive(Parser)]
struct Args {
    target: String,
}

fn main() {
    let args = Args::parse();
    let mut sth = StreamTransmissionHeader {
        version: 0, flags: 0, length: 0
    };

    let mut stream = match TcpStream::connect(&args.target) {
        Ok(stream) => {
            println!("Connecté à {}", args.target);
            stream
        }
        Err(e) => {
            eprintln!("Connexion à {} impossible : {}", args.target, e);
            std::process::exit(1);
        }
    };

    println!("Message: {:?}", SSRMMSG);

    sth.decode(&mut stream).unwrap();

    println!("version: {} | flags: {} | length: {}", sth.version, sth.flags, sth.length);

    stream.shutdown(Shutdown::Both).expect("shutdown");

    println!("Déconnecté de {}", args.target);
}
