use tokio::sync::oneshot;

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
}