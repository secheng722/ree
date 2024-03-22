use std::{net::SocketAddr, sync::Arc};

use http_body_util::combinators::BoxBody;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use ree::{
    handler::{full, ree_request},
    ree::Ree,
};
use tokio::net::TcpListener;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut ree = Ree::new();
    ree.get("/", hello);
    ree.get("/hello2", hello2);
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&socket_addr).await?;
    let engine = Arc::new(ree);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let engine = engine.clone();
        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| ree_request(req, engine.clone())))
                .await
            {
                eprintln!("server error: {}", err);
            }
        });
    }
}
