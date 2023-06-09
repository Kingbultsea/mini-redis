#[tokio::main]
async fn main() {
    // spawn生成的任务会首先提交给调度器，然后由它负责调度执行。需要注意的是，执行任务的线程未必是创建任务的线程
    // 任务完全有可能运行在另一个不同的线程上，而且任务在生成后，它还可能会在线程间被移动

    // tokio::spawn 生成的任务必须实现 Send 特征，因为当这些任务在 .await 执行过程中发生阻塞时，Tokio 调度器会将任务在线程间移动
    // 一个任务要实现 Send 特征，那它在 .await 调用的过程中所持有的全部数据都必须实现 Send 特征
    let handle = tokio::spawn(async {
       1
    });

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
