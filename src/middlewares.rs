use hyper::body::Incoming;
use hyper::Request;

pub fn is_authenticated(req: &Request<Incoming>) -> bool {
    return match req.headers().get("mj_is_goat") {
        None => false,
        Some(v) => match v.to_str() {
            Ok(v) => v.to_lowercase() == "true",
            Err(_) => false,
        },
    };
}
