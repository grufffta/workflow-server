use std::fs;
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
    #[arg(default_value_t = 5080, value_parser = port_in_range)]
    port: u16,
    /// Bind to specified IP address
    #[arg(default_value_t = IpAddr::from_str("0.0.0.0").unwrap(), value_parser = is_valid_ip_address)]
    address: IpAddr,
}

/// starts the server processes
pub(crate) fn start_server(args: &StartCommandArgs, verbose: u8) {
    println!("\n  Starting server on {}:{} verbosity level {}", args.address, args.port, verbose);
    let bind_result = TcpListener::bind((args.address, args.port));
    match bind_result {
        Ok(listener) => {
            println!("  Listener Started\n");
            for incoming in listener.incoming() {
                match incoming {
                    Ok(stream) => handle_http_connection(stream, verbose),
                    Err(e) => println!("Unable to establish stream: {}", e.to_string()),
                }
            }
        }
        Err(e) => println!("\nUnable to start server: {}", e.to_string()),
    }
}

/// handles a HTTP TCP connection for a listener
fn handle_http_connection(mut stream: TcpStream, verbose: u8) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if verbose > 0 {
        println!("Requests {:#?}", request);
    }

    let filename = get_template_filename(request);

    match fs::read_to_string(format!("www/{filename}")) {
        Ok(contents) => {
            let response = get_response(contents);
            if verbose > 0 {
                println!("Response ------\n{:#}", response);
            }
            stream.write_all(response.as_bytes()).expect("Unable to write to stream")
        }
        Err(e) => println!("Unable to read '{filename}': {}\n", e.to_string())
    }
}

/// gets the appropriate HTML file for a route
fn get_template_filename(request: Vec<String>) -> &'static str {
    return match request.first().unwrap() {
        method => {
            if method == "GET / HTTP/1.1" {
                "up.html"
            } else {
                "404.html"
            }
        }
    };
}

/// formats a http response
fn get_response(contents: String) -> String {
    let status = "HTTP/1.1 200 OK";
    let content_len = contents.len();
    let headers = format!("{status}\r\nContent-Length: {content_len}\r\n");
    format!("{headers}\r\n{contents}")
}

