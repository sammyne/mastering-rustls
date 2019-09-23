use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{self, Session};

fn new_config(cert_path: &str) -> Result<Arc<rustls::ClientConfig>, String> {
    let certs_pem = match fs::File::open(cert_path) {
        Err(err) => return Err(err.to_string()),
        Ok(v) => v,
    };

    let mut reader = io::BufReader::new(certs_pem);

    let mut config = rustls::ClientConfig::new();
    config.root_store.add_pem_file(&mut reader).unwrap();

    Ok(Arc::new(config))
}

fn main() {
    const CA_PATH: &str = "./server.crt";

    let config = new_config(CA_PATH).unwrap();
    let domain_name = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();

    let mut session = rustls::ClientSession::new(&config, domain_name);
    let mut socket = TcpStream::connect("localhost:4433").unwrap();

    let mut client = rustls::Stream::new(&mut session, &mut socket);
    println!("one");
    client.write(b"hello world").unwrap();

    let ciphersuite = client.sess.get_negotiated_ciphersuite().unwrap();
    println!("2");
    println!("Current ciphersuite: {:?}", ciphersuite.suite);

    let mut plaintext = Vec::new();
    println!("3");
    client.read_to_end(&mut plaintext).unwrap();
    println!("4");
    io::stdout().write_all(&plaintext).unwrap();
}
