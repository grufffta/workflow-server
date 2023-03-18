# Workflow Engine

This repository hosts a project used to learn various concepts in Rust.

The binary will be a server used to receive, send and process files over various transports.
You can also use the binary to test and configure the server using a CLI/TUI.

## Build and Run

Run `cargo run` to explore the help adn get started.

## Transports

### AS2 Transport

Files are transferred using the [AS2 RFC4130](https://www.ietf.org/rfc/rfc4130.txt) specification.
This specifies away to securely transfer files over HTTP, the ability to authenticate the sender
and indicate a file has been received in its entirety.

### HTTP Transport

### SFTP Transport

### FTP

