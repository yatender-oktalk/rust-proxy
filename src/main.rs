use http_body_util::Full;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn proxy_handler(_req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    println!("Received HTTP Request: {} {}", _req.method(), _req.uri());

    Ok(Response::new(Full::new(Bytes::from(
        "Hello from Rust Reverse Proxy!",
    ))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "127.0.0.1:8080".parse().expect("LOL it's not available");

    let listener = TcpListener::bind(address).await?;
    println!("🚀 Reverse Proxy running on http://{}", address);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        println!("{:?}", stream);
        println!("Received connection from {}", client_addr);

        tokio::spawn(async move {
            let io = TokioIo::new(stream);

            let service = service_fn(proxy_handler);

            if let Err(err) = auto::Builder::new(TokioExecutor::new())
                .serve_connection(io, service)
                .await
            {
                eprintln!("Error serving connection from {}: {}", client_addr, err);
            }
        });
    }
}
