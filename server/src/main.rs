use std::fs;
use std::io::{self, Write};
use std::net::{self, TcpListener};
use std::sync::Arc;

use rustls;

//struct Server {
//    tls_config: Arc<rustls::ServerConfig>,
//    tcp_listener: TcpListener,
//}

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

/*
fn new(cert_path: &str, key_path: &str, listen_uri: &str) -> Result<Server, ConnectionError> {
    fn open_cert_file<F, T>(file: &str, method: F) -> Result<Vec<T>, ConnectionError>
    where
        F: Fn(&mut dyn io::BufRead) -> Result<Vec<T>, ()>,
    {
        let certfile = fs::File::open(file)?;
        let mut reader = io::BufReader::new(certfile);
        match method(&mut reader) {
            Err(_) => return Err(ConnectionError::failed_cert(file)),
            Ok(certs) => match certs.len() {
                0 => return Err(ConnectionError::failed_cert(file)),
                _ => Ok(certs),
            },
        }
    }

    let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());

    let certs = open_cert_file(cert_path, rustls::internal::pemfile::certs)?;
    let key = open_cert_file(key_path, rustls::internal::pemfile::rsa_private_keys)
        .or_else(|_| open_cert_file(key_path, rustls::internal::pemfile::pkcs8_private_keys))
        .and_then(|keys| match keys.get(0) {
            None => Err(ConnectionError::failed_cert(key_path)),
            Some(key) => Ok(key.clone()),
        })?;

    config.set_single_cert(certs, key)?;

    let listener = TcpListener::bind(listen_uri)?;

    Ok(Server {
        tls_config: Arc::new(config),
        tcp_listener: listener,
    })
}
*/

fn main() {
    const CERT_PATH: &str = "./server.crt";
    const KEY_PATH: &str = "./server.key";

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

        tls_stream.write(b"OK\r\n").unwrap();
    }
}
