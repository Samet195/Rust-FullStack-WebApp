#![cfg(feature = "tls")]

use std::io::{BufReader, Cursor};

use rustls::ServerConfig;
use rustls_pemfile::{certs, private_key};

pub(crate) fn load_config() -> ServerConfig {
    let cert_file = &mut BufReader::new(Cursor::new(include_bytes!("../../certs/cert.pem")));
    let key_file = &mut BufReader::new(Cursor::new(include_bytes!("../../certs/key.pem")));

    let certs = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let private_key = private_key(key_file).unwrap().unwrap();

    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .unwrap()
}
