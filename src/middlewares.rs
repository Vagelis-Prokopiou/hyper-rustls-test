use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{body::Bytes, Request, Response};

pub fn is_authenticated(req: &Request<Incoming>) -> bool {
    return match req.headers().get("mj_is_goat") {
        None => false,
        Some(v) => match v.to_str() {
            Ok(v) => v.to_lowercase() == "true",
            Err(_) => false,
        },
    };
}

pub fn response_middleware_1(response: Response<Full<Bytes>>) -> Response<Full<Bytes>> {
    let mut r = Response::new(Full::from("aa"));
    *r.headers_mut() = response.headers().clone();
    *r.status_mut() = response.status();

    match response.headers().get("auth") {
        Some(v) => {
            r.headers_mut()
                .insert("response_middleware_1", "run".parse().unwrap());
        }
        None => {}
    };

    *r.body_mut() = Full::from(response.into_body());

    //let mut _r = response;
    return r;
}
