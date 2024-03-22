use std::{net::SocketAddr, sync::Arc};

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    service::service_fn,
    Method, Request, Response,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub type Handler =
    fn(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;

type Router = (Method, String, Handler);

#[derive(Debug, Clone)]
struct Engine {
    routers: Vec<Router>,
}

fn new() -> Engine {
    Engine {
        routers: Vec::new(),
    }
}

fn add_route(engine: &mut Engine, method: Method, path: &str, handler: Handler) {
    engine.routers.push((method, path.to_string(), handler));
}

fn get(engine: &mut Engine, path: &str, handler: Handler) {
    add_route(engine, Method::GET, path, handler);
}

fn post(engine: &mut Engine, path: &str, handler: Handler) {
    add_route(engine, Method::POST, path, handler);
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
fn hello(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::new(full("Try POSTing data to /echo")))
}
fn hello2(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::new(full("hello from /hello")))
}

async fn handler(
    req: Request<hyper::body::Incoming>,
    engine: Arc<Engine>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    if let Some(f) = engine.routers.iter().find_map(|(method, path, handler)| {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut engine = new();
    get(&mut engine, "/", hello);
    get(&mut engine, "/hello", hello2);
    let engine = Arc::new(engine);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let engine_ref = engine.clone();
        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| handler(req, engine_ref.clone())))
                .await
            {
                eprintln!("server error: {}", err);
            }
        });
    }
}
