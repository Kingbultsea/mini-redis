use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

// select只需要遵循借用规则，因为表达式只会在同一线程上运行，不需要所有权。
async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    tokio::select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        else => {}
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    let data = [1, 1, 1];
    let _ = race(
        &data,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
    )
    .await;
}
