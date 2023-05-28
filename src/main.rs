#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
       10086
    });

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
