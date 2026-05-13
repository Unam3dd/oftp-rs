use clap::Parser;
use std::net::{TcpStream, Shutdown};
use std::io::Read;
use oftp_rs::dbg_hex;

#[derive(Parser)]
struct Args {
    target: String
}


fn main() {
    let args = Args::parse();

    let mut buffer: [u8; 1024] = [0; 1024];
    
    let mut stream = match TcpStream::connect(&args.target) {
        Ok(stream) => {
            println!("Connected to {}", args.target);
            stream
        },
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", args.target, e);
            std::process::exit(1);
        }
    };

    stream.read(&mut buffer).expect("Failed to read from stream");

    dbg_hex!(&buffer[..32]);

    println!("Received data: {}", String::from_utf8_lossy(&buffer));

    stream
        .shutdown(Shutdown::Both)
        .expect("Failed to shutdown stream");

    println!("Disconnected from {}", args.target);
}