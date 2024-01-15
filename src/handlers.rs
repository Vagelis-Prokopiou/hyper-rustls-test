//! Simple HTTPS echo service based on hyper_util and rustls
//!
//! First parameter is the mandatory port to use.
//! Certificate and private key are hardcoded to sample files.
//! hyper will automatically use HTTP/2 if a client starts talking HTTP/2,
//! otherwise HTTP/1.1 will be used.

use crate::middlewares::is_authenticated;
use crate::response_bad_request;
use http::{Method, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};

// Custom echo service, handling two different routes and a
// catch-all 404 responder.
pub async fn root_handler(
    mut req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let mut response = Response::new(Full::default());

    // Request middlewares.
    if !is_authenticated(&req) {
        return Ok(response_bad_request(Some("Auth failed")));
    }
    // Request middlewares end

    match (req.method(), req.uri().path()) {
        // Help route.
        (&Method::GET, "/") => {
            *response.body_mut() = Full::from("Try POST /echo\n");
        }
        // Echo service route.
        (&Method::POST, "/echo") => {
            *response.body_mut() = Full::from(req.into_body().collect().await?.to_bytes());
        }
        // Catch-all 404.
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}
