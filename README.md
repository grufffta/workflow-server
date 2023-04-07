# Workflow Automation Engine

This repository hosts a project used to learn various concepts in Rust.

The goal of the project is to create a server that can accept files over various transports
and forward these to a handler, or proxy the request to another location, potentially
transforming the request as part of the pipe line.

Initially there will be a web API and a CLI/TUI to interact with the server and perform
common tasks.

## Build and Run

Run `cargo run` to explore the help and get started.

## Transports

The server should be capable of receiving a file on one transport and delivering it across another.

### AS2 Transports

Files are transferred using the [AS2 RFC4130](https://www.ietf.org/rfc/rfc4130.txt) specification.
This specifies away to securely transfer files over HTTP, the ability to authenticate the sender
and indicate that the file has been received in its entirety.

### HTTP Transports

### FTP/S Transport

Receive files from or send files to an FTP server optionally using FTP/S

### SFTP Transport

Receive files across a SFTP connection using public/private key and/or password authentication.
