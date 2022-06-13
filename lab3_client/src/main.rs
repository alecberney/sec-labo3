/// This file is used to configure and start a TLS connection to the server.
/// On new connections, the `client` function is called.
///
/// Tasks: - Configure the TLS client properly.

mod connection;
mod action;
mod input_handlers;

use std::error::Error;
use std::fs::File;
use native_tls::{Certificate, Protocol, TlsConnector};
use std::io::{Read};
use std::net::TcpStream;
use read_input::prelude::*;
use crate::action::Action;
use crate::connection::Connection;

// Called once connected to the server, used to execute actions.
fn client(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    loop {
        let banner = conn.receive::<String>()?;
        println!("{}", banner);

        Action::display();
        let action = input::<Action>().msg("Please select: ").get();

        action.perform(conn)?;
        println!();
    }
}

// Load a PEM certificate
fn load_server_cert(cert_file: &str) -> Certificate {
    let mut cert = Vec::new();
    let mut cert_file = File::open(cert_file).unwrap();

    cert_file.read_to_end(&mut cert).unwrap();
    Certificate::from_pem(&cert).unwrap()
}

const SERVER_HOST: &str = "localhost";
const SERVER_PORT: &str = "4444";
const SERVER_CERT_PATH: &str = "../lab3_server/keys/rsa_cert.pem";

fn main() {
    let connector = TlsConnector::builder()
        .add_root_certificate(load_server_cert(SERVER_CERT_PATH))
        .min_protocol_version(Some(Protocol::Tlsv12))
        .max_protocol_version(None)
        .disable_built_in_roots(true)
        .danger_accept_invalid_certs(false)
        .danger_accept_invalid_hostnames(false)
        .build()
        .expect("Failed to build TlsConnector");

    let stream = match TcpStream::connect(format!("{}:{}", SERVER_HOST, SERVER_PORT)) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return;
        }
    };

    let stream = match connector.connect(SERVER_HOST, stream) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to init TLS: {}", e);
            return;
        }
    };

    let mut conn = Connection::new(stream);
    if let Err(e) = client(&mut conn) {
        eprintln!("{}", e);
    }
}
