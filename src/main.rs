use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{
    body::{Body, Bytes},
    server::conn::http1,
    service::service_fn,
    Error, Method, Request, Response, StatusCode,
};
use hyper_util::{client::legacy::Error, rt::TokioIo};
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Arc<dyn Body<Data = Bytes, Error = hyper::Error>>>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let result = hello(req).await;
            match result {
                Ok(response) => Ok(response),
                Err(_) => Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(empty())
                    .unwrap()),
            }
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(empty())
            .unwrap()),
    }
}

fn empty() -> BoxBody<Bytes, Error> {
    Empty::new().map_err(|never| match never {}).boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("server error: {}", err);
            }
        });
    }
}
