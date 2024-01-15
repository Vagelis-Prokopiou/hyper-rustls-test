//! Simple HTTPS echo service based on hyper_util and rustls
//!
//! First parameter is the mandatory port to use.
//! Certificate and private key are hardcoded to sample files.
//! hyper will automatically use HTTP/2 if a client starts talking HTTP/2,
//! otherwise HTTP/1.1 will be used.

use hyper::service::service_fn;
use hyper_rustls_test::{handlers::root_handler, *};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use rustls::ServerConfig;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

fn main() {
    // Serve an echo service over HTTPS, with proper error handling.
    if let Err(e) = run_server() {
        eprintln!("FAILED: {}", e);
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // First parameter is port number (optional, defaults to 1337)
    let port = match env::args().nth(1) {
        Some(ref p) => p.parse()?,
        None => 1337,
    };
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
    let certs = load_certs("certs/server.crt")?;
    let key = load_private_key("certs/server.key")?;

    // Create a TCP listener via tokio.
    let incoming = TcpListener::bind(&addr).await?;

    // Build TLS configuration.
    let mut server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| error(e.to_string()))?;
    server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];

    let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));
    let service = service_fn(root_handler);

    println!("Starting to serve on https://{}", addr);
    loop {
        let (tcp_stream, _remote_addr) = incoming.accept().await?;

        let tls_acceptor = tls_acceptor.clone();
        tokio::spawn(async move {
            let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                Ok(tls_stream) => tls_stream,
                Err(err) => {
                    eprintln!("failed to perform tls handshake: {err:#}");
                    return;
                }
            };
            if let Err(err) = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(tls_stream), service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }
}
