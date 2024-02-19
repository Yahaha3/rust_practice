// use tokio::net::ToSocketAddrs;
// use tokio::runtime::Runtime;

// pub use crate::client::Message;

// pub struct BlockingClient {
//     inner: crate::client::Client,

//     rt: Runtime,
// }

// pub fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<BlockingClient> {
//     let rt = tokio::runtime::Builder::new_current_thread()
//     .enable_all().build()?;

// // 使用block_on在同步代码中调用异步代码
//     let inner = rt.block_on(crate::client::connect(addr))?;

//     Ok(BlockingClient {inner, rt})
// }

// 在同步任务中执行异步任务并等待异步结果
use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};

fn main() {
    let runtime = Builder::new_multi_thread().worker_threads(1).enable_all().build().unwrap();

    let mut handles = Vec::with_capacity(10);

    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_tk(i)));
    }

    std::thread::sleep(Duration::from_millis(10));
    println!("Finished time-consuming task");
    // 等待异步结果
    for handle in handles {
        runtime.block_on(handle).unwrap();
    }
}

async fn my_bg_tk(i: u64) {
    let millis = 1000 - 50*i;

    println!("task {} sleeping for {} ms", i, millis);

    sleep(Duration::from_millis(millis)).await;

    println!("task {} stoping", i);
}