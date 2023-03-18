use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::str::FromStr;

use clap::Args;

use crate::commands::validators::{
    is_valid_ip_address,
    port_in_range,
};

#[derive(Args)]
pub(crate) struct StartCommandArgs {
    /// Start server on specified port
    #[arg(default_value_t = 5000, value_parser = port_in_range)]
    port: u16,
    /// Bind to specified IP address
    #[arg(default_value_t = IpAddr::from_str("0.0.0.0").unwrap(), value_parser = is_valid_ip_address)]
    address: IpAddr,
}

pub(crate) fn start_server(args: &StartCommandArgs) {
    println!("\n  Starting server on {}:{}", args.address, args.port);
    let bind_result = TcpListener::bind((args.address, args.port));
    match bind_result {
        Ok(listener) => {
            println!("  Listener Started");
            for incoming in listener.incoming() {
                match incoming {
                    Ok(stream) => handle_connection(stream),
                    Err(e) => println!("Unable to establish stream: {}", e.to_string()),
                }
            }
        }
        Err(e) => println!("\nUnable to start server: {}", e.to_string()),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request : Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();

    println!("Requests {:#?}", request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    println!("Response:\n    {}", response);

    stream.write_all(response.as_bytes()).expect("Unable to write to stream")
}