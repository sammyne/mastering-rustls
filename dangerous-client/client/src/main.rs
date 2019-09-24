use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{self, Session};

struct ToyServerCertVerifier {}

impl ToyServerCertVerifier {
    fn new() -> Self {
        ToyServerCertVerifier {}
    }
}

impl rustls::ServerCertVerifier for ToyServerCertVerifier {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        _presented_certs: &[rustls::Certificate],
        dns_name: webpki::DNSNameRef,
        _ocsp_response: &[u8],
    ) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
        let localhost = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();
        if localhost.to_owned() == dns_name.to_owned() {
            // localhost is whitelisted
            return Ok(rustls::ServerCertVerified::assertion());
        }

        Err(rustls::TLSError::WebPKIError(
            webpki::Error::ExtensionValueInvalid,
        ))
    }
}

fn new_config() -> Arc<rustls::ClientConfig> {
    let mut config = rustls::ClientConfig::new();

    let verifier = Arc::new(ToyServerCertVerifier::new());
    config.dangerous().set_certificate_verifier(verifier);

    Arc::new(config)
}

fn main() {
    let config = new_config();
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
