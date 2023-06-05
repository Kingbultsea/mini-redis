use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        println!("connect");

        tokio::spawn(async move {
            // 当任务因为调度在线程间移动时，存储在栈上的数据需要进行保存和恢复，过大的栈上变量会带来不小的数据拷贝开销 因此，存储大量数据的变量最好放到堆上
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // 返回值 `Ok(0)` 说明对端已经关闭
                    Ok(0) => return,
                    Ok(n) => {
                        println!("有数据进来, {}", n);
                        // Copy the data back to socket
                        // 将数据拷贝回 socket 中
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // 非预期错误，由于我们这里无需再做什么，因此直接停止处理
                            println!("错误");
                            return;
                        }
                    }
                    Err(_) => {
                      // 非预期错误，由于我们无需再做什么，因此直接停止处理
                        return;
                    }
                }
            }
        });
    }
}
