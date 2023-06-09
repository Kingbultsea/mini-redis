use std::{io, sync::Arc, thread, time::Duration};

use dashmap::DashMap;
use my_mini_redis::process;
use tokio::{net::TcpListener, sync::oneshot};

async fn connect_mth() -> io::Result<()> {
    let (tx, rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        thread::sleep(Duration::from_millis(50));

        tx.send(()).unwrap();
    });

    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        _ = async {
            loop {
                let (socket, _) = listener.accept().await?;
                let db = Arc::new(DashMap::new());

                tokio::spawn(async move { process(socket, db) });
                println!("CONNECT");
            }

            // 推断
            Ok::<_, io::Error>(())
        } => {}
        _ = rx => {
            println!("terminating accept loop");
        }
    }

    println!("wait select macro done");

    Ok(())
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("1");
    });

    tokio::spawn(async {
        let _ = tx2.send("2");
    });

    // 任何一个分支完成后，没有被执行的分支将会被drop
    tokio::select! {
        val = rx1 => {
            println!("rx1{:?}", val);
        }
        val = rx2 => {
            println!("rx2 {:?}", val);
        }
    }

    connect_mth().await;
}
