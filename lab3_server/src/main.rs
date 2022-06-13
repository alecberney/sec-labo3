/// This file is used to configure and start the TLS server.
/// On new connections, the `handle_client` function is called in a thread
///
/// Tasks: - Configure the TLS server properly.
///        - Log stuff whenever required
mod action;
mod connection;
mod database;
mod user;
mod hashing_tools;
mod messages;
mod access_control;
mod user_connected;
mod env_reader;

use crate::action::Action;
use crate::user_connected::ConnectedUser;
use crate::user::UserRole;
use crate::database::Database;
use crate::env_reader::read_env_file;
use connection::Connection;
use lazy_static::lazy_static;
use native_tls::{Identity, Protocol, TlsAcceptor};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use simplelog::{ColorChoice, Config, LevelFilter, TerminalMode, TermLogger};
use log::{error, info, trace, warn};

lazy_static! {
    static ref MOTIVATIONAL_QUOTES: Vec<&'static str> = vec![
        "Train people well enough so they can leave. Treat them well enough so they don’t want to.",
        "Human Resources isn’t a thing we do. It’s the thing that runs our business.",
        "When people go to work, they shouldn’t have to leave their hearts at home.",
        "Hire character. Train skill.",
        "Every problem is a gift - without problems we would not grow.",
        "Far and away the best prize that life offers is the chance to work hard at work worth doing.",
        "Believe you can and you’re halfway there."
    ];
}

// Handles client connection by sending a banner and then waiting for a client action
fn handle_client(conn: Connection) -> Result<(), Box<dyn Error>> {
    trace!("Handling new client");

    let mut u = ConnectedUser::anonymous(conn); // Anonymous user at first
    loop {
        let mut banner = "Welcome to RESIGN (hR onlinE uSer dIrectory manaGemeNt)!".to_string();
        if !u.is_anonymous() {
            banner.push_str(
                format!("\nCurrently logged in as {}", u.user_account()?.username()).as_str(),
            );

            if let UserRole::HR = u.user_account()?.role() {
                let quote =
                    MOTIVATIONAL_QUOTES[rand::thread_rng().gen_range(0..MOTIVATIONAL_QUOTES.len())];
                banner.push_str(format!("\nQuote of the day: {}\n", quote).as_str());
            }
        }

        // We send the banner to  the client and we expect to receive an Action
        u.conn().send(&banner)?;
        let action = u.conn().receive::<Action>()?;
        action.perform(&mut u)?;
    }
}

// Load the server certificate and private key from PKCS8 format
fn load_server_identity(cert_file: &str, key_file: &str) -> Identity {
    let mut cert = Vec::new();
    let mut key = Vec::new();

    // No log cause the server crashes if it doesn't work
    let mut cert_file = File::open(cert_file).expect("Certificate file not found");
    let mut key_file = File::open(key_file).expect("Key file not found");

    cert_file.read_to_end(&mut cert).unwrap();
    key_file.read_to_end(&mut key).unwrap();

    Identity::from_pkcs8(&cert, &key).unwrap()
}

// Create a new TLS configuration
fn tls_config(cert_file: &str, key_file: &str) -> Arc<TlsAcceptor> {
    let identity = load_server_identity(cert_file, key_file);

    // No log cause the server crashes if it doesn't work
    let acceptor = TlsAcceptor::builder(identity)
        .min_protocol_version(Some(Protocol::Tlsv12))
        .max_protocol_version(None)
        .build()
        .expect("Could not build TlsAcceptor");

    Arc::new(acceptor)
}

fn main() {
    // Initialize logging policy
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto
    ).unwrap();

    // Add default account in DB if file is not present
    Database::init();

    // Get config infos from env file
    let config = match read_env_file() {
        Ok(config) => config,
        Err(e) => {
            error!("An error occurred reading env file: {}", e);
            panic!("An error occurred reading env file: {}", e)
        }
    };

    // Start TLS server and wait for new connections
    let acceptor = tls_config(&config.certificate_path, &config.key_path);
    let listener = TcpListener::bind(config.server_ip).unwrap();
    //println!("Server started");
    info!("Server started");

    // Handles new connection, negotiate TLS and call handle_client
    for stream in listener.incoming() {
        info!("New connection");
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    trace!("TLS handshake");
                    // TLS handshake on top of the connection using the TlsAcceptor
                    let stream = acceptor.accept(stream);
                    if stream.is_err() {
                        warn!("TLS handshake failed with error: {}", stream.err().unwrap());
                    } else {
                        info!("TLS client connection accepted");
                        if let Err(e) = handle_client(Connection::new(stream.unwrap())) {
                            info!("Connection closed: {}", e);
                            return;
                        }
                    }
                });
            }
            Err(e) => {
                warn!("Connection failed with error: {}", e);
            }
        }
    }

    info!("Server stopped");
}
