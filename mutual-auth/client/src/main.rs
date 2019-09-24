use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{self, Session};

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
        Ok(keys) => keys,
        Err(_) => return Err("failed to read key".to_string()),
    };

    Ok(keys[0].clone())
}

fn main() {
    const CA_CERT_PATH: &str = "../../pki/ca.cert";
    const CLIENT_CERT_PATH: &str = "../../pki/client.cert";
    const KEY_PATH: &str = "../../pki/client.key";

    let ca_certs = read_certs(CA_CERT_PATH).unwrap();
    let client_certs = read_certs(CLIENT_CERT_PATH).unwrap();
    let key = read_private_key(KEY_PATH).unwrap();

    let config = {
        let mut c = rustls::ClientConfig::new();
        c.root_store.add(&ca_certs[0]).unwrap();
        c.set_single_client_cert(client_certs, key);
        Arc::new(c)
    };

    let domain_name = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();

    let mut session = rustls::ClientSession::new(&config, domain_name);
    let mut socket = TcpStream::connect("localhost:4433").unwrap();

    let mut client = rustls::Stream::new(&mut session, &mut socket);
    client.write(b"hello world").unwrap();
    client.flush().unwrap();

    let ciphersuite = client.sess.get_negotiated_ciphersuite().unwrap();
    println!("Current ciphersuite: {:?}", ciphersuite.suite);

    let mut plaintext = Vec::new();
    client.read_to_end(&mut plaintext).unwrap();
    io::stdout().write_all(&plaintext).unwrap();
}
