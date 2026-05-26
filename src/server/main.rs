use std::io::{Read, Write, stdout};
use std::net::{TcpListener, TcpStream};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "IRC server written in Rust")]
struct CLI {
    addr: String,
    port: u16,
}

fn main() -> std::io::Result<()> {
    let cli = CLI::parse();
    let addr = get_addr(&cli.addr);
    let full_addr = format!("{}:{}", addr, cli.port);

    let listener = TcpListener::bind(full_addr)?;
    println!("Server listening on: {}", addr);

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}

fn get_addr(arg_addr: &str) -> &str {
    let addr = match arg_addr{
        "localhost" => "127.0.0.1",
        _ => arg_addr,
    };
    return addr;
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{ 
    println!("Connection Accepted");

    let mut buf = [0; 128];

    loop {
        let read_bytes = match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        stdout().write(&buf[0..read_bytes])?;
        stdout().flush()?;
    }

    println!("Connection Lost");
    Ok(())
}
