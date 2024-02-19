use std::collections::btree_map::Values;

use tokio::sync::oneshot;
use tokio::sync::mpsc;

async fn operation() -> String{
    String::from("operation")
}

#[tokio::main]
async fn main() {

    // let op = operation();
    // // ↓ 以下操作是将Future固定，用于保证await执行
    // tokio::pin!(op);


    //     loop {
    //         let (mut tx1,rx1) = oneshot::channel();
    //         let (tx2, rx2) = oneshot::channel();
    //         tokio::select! {
    //             // &mut op 的形式可以保证每次都调用同一个实例，而不是新生成一个实例
    //             val = &mut op => {
    //                 let _ = tx1.send(val);
    //             }
    //             _ = tx1.closed() => {
    //                 // 此时会关闭operation任务
    //             }
    //         }
    //             tokio::spawn(async {
    //     let _ = tx2.send("two");
    // });

    //             // select可以用于同时执行多个异步任务，等待其中一个返回
    // tokio::select! {
    //     val = rx1 => {
    //         println!("rx1 completed first with {:?}", val);
    //     }

    //     val = rx2 => {
    //         println!("rx2 completed first with {:?}", val);
    //     }
    //     // else => {}
    // }
    //     }
    test().await;

}

async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(input) => input,
        None => return None,
    };

    Some("Success".to_string())
}

async fn test() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }

            Some(v) = rx.recv() => {
                
                println!("action = {}", v);
                if v % 2 == 0 {
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        }
    }
}