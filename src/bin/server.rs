use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::sync::Arc;
use dashmap::DashMap;

use my_mini_redis::process;

type Db = Arc<DashMap<String, Bytes>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(DashMap::new());

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // 将 handle 克隆一份
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
