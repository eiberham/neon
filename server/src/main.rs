use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};

use std::io;

async fn process<T: AsyncReadExt + Unpin>(mut socket: T) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = socket.read(&mut buf).await?;

    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:1234").await?; // 6379 is the default redis port

    loop {
        let (socket, _) = listener.accept().await?;
        process(socket).await;
    }
}
