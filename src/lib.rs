use bytes::Bytes;
use dashmap::DashMap;
use mini_redis::{cmd::Publish, Connection, Frame};
use std::{sync::Arc, vec};
use tokio::net::TcpStream;
// tokio::sync::Mutex

type Db = Arc<DashMap<String, Bytes>>;

pub async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Publish, Set, Subscribe};

    // `mini-redis` 提供的便利函数，使用返回的 `connection` 可以用于从 socket 中读取数据并解析为数据帧
    let mut connection = Connection::new(socket);

    // 使用 `read_frame` 方法从连接获取一个数据帧：一条redis命令 + 相应的数据
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 值被存储为 `Vec<u8>` 的形式
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` 期待数据的类型是 `Bytes`， 该类型会在后面章节讲解，
                    // 此时，你只要知道 `&Vec<u8>` 可以使用 `into()` 方法转换成 `Bytes` 类型
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            Subscribe(_) => {
                let c = vec![Frame::Bulk("subscribe".into()), Frame::Bulk("numbers".into())];
                Frame::Array(c)
            },
            Publish(cmd) => {
                Frame::Integer(12)
            },
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // 将请求响应返回给客户端
        connection.write_frame(&response).await.unwrap();
    }
}
