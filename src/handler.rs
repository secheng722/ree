use std::sync::Arc;

use http_body_util::combinators::BoxBody;
use hyper::{
    body::{Bytes, Incoming},
    Request, Response,
};

use http_body_util::{BodyExt, Empty, Full};

use crate::ree::Ree;

pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
pub type Handler =
    fn(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;

pub async fn ree_request(
    req: Request<hyper::body::Incoming>,
    engine: Arc<Ree>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    if let Some(f) = engine.routers().iter().find_map(|(method, path, handler)| {
        if req.method() == *method && req.uri().path() == path {
            Some(handler)
        } else {
            None
        }
    }) {
        return f(req);
    } else {
        return Ok(Response::builder()
            .status(404)
            .body(empty())
            .expect("response builder with known status code must not fail"));
    };
}
