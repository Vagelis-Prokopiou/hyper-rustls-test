pub mod handlers;
pub mod middlewares;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use std::fs;
use std::io;

fn response_bad_request(message: Option<&str>) -> Response<Full<Bytes>> {
    let body = message.unwrap_or_default().to_string();
    let mut response = Response::new(Full::from(body));
    *response.status_mut() = hyper::StatusCode::BAD_REQUEST;
    return response;
}

pub fn error(err: String) -> io::Error {
    return io::Error::new(io::ErrorKind::Other, err);
}

// Load public certificate from file.
pub fn load_certs(filename: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    return rustls_pemfile::certs(&mut reader).collect();
}

// Load private key from file.
pub fn load_private_key(filename: &str) -> io::Result<PrivateKeyDer<'static>> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    return rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap());
}
