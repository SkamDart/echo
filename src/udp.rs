use tokio;
use tokio::net::UdpSocket;

use std::env;
use std::error::Error;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:4000".to_string());
    let mut sock = UdpSocket::bind(&addr).await?;
    let mut to_send: Option<(usize, SocketAddr)> = None;
    let mut buf = vec![0; 1024];

    loop {
        if let Some((size, peer)) = to_send {
            let _amt = sock.send_to(&buf[..size], &peer).await?;
            println!("{}", std::str::from_utf8(&buf).unwrap_or("this is not working"));
        }
        to_send = Some(sock.recv_from(&mut buf).await?);
    }
}
