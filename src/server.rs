use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:6142".to_string());
    let mut listener = TcpListener::bind(&addr).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf: [u8; 1024] = [0; 1024];
            loop {
                let n = socket.read(&mut buf).await.expect("socket read failed");
                if n == 0 {
                    return;
                }
                socket.write_all(&buf[0..n]).await.expect("write fail");
            }
        });
    }
}
