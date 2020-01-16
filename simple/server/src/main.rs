use std::fs;
use std::io::{self, Read, Write};
use std::net::{self, TcpListener};
use std::sync::Arc;

use rustls;

fn read_certs(path: &str) -> Result<Vec<rustls::Certificate>, String> {
    let data = match fs::File::open(path) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    let mut reader = io::BufReader::new(data);

    match rustls::internal::pemfile::certs(&mut reader) {
        Err(_) => Err("failed to read out cert".to_string()),
        Ok(certs) => match certs.len() {
            0 => return Err("no cert".to_string()),
            _ => Ok(certs),
        },
    }
}

// @dev the header in PEM block of key must be "BEGIN RSA PRIVATE KEY"
fn read_private_key(path: &str) -> Result<rustls::PrivateKey, String> {
    let key_pem = match fs::File::open(path) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    let mut reader = io::BufReader::new(key_pem);

    let keys = match rustls::internal::pemfile::rsa_private_keys(&mut reader) {
        //let keys = match rustls::internal::pemfile::pkcs8_private_keys(&mut reader) {
        Ok(keys) => keys,
        Err(_) => return Err("failed to read key".to_string()),
    };

    Ok(keys[0].clone())
}

fn main() {
    const CERT_PATH: &str = "../../pki/server.cert";
    const KEY_PATH: &str = "../../pki/server.key";

    let certs = read_certs(CERT_PATH).unwrap();
    let key = read_private_key(KEY_PATH).unwrap();

    let config = {
        let mut c = rustls::ServerConfig::new(rustls::NoClientAuth::new());
        c.set_single_cert(certs, key).unwrap();
        Arc::new(c)
    };

    let addr: net::SocketAddr = "0.0.0.0:4433".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    for stream in listener.incoming().take(1) {
        let mut socket = stream.unwrap();
        let mut session = rustls::ServerSession::new(&config);

        let mut tls_stream = rustls::Stream::new(&mut session, &mut socket);

        let mut plaintext = [0; 128];
        tls_stream.read(&mut plaintext).unwrap();

        io::stdout().write_all(&plaintext).unwrap();

        tls_stream.write(b"OK from server\r\n").unwrap();
        tls_stream.flush().unwrap();
    }
}
