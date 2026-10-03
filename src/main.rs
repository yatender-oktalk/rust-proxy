use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "127.0.0.1:8080".parse().expect("LOL it's not available");

    let listener = TcpListener::bind(address).await?;
    println!("🚀 Reverse Proxy running on http://{}", address);

    loop {
        let (_, client_addr) = listener.accept().await?;
        println!("Received connection from {}", client_addr);
    }
}
