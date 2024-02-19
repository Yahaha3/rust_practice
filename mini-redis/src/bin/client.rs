use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    // let (ty, ry) = oneshot::channel();
    // let t1 = tokio::spawn(async move{
    //     let res = client.get("hello").await;
    // });

    // let t2 = tokio::spawn(async move{
    //     client.set("foo", "bar".into()).await;
    // });

    // t1.await.unwrap();
    // t2.await.unwrap();

    let t1 = tokio::spawn(async move {
        let (rtx, rrx) = oneshot::channel();
        let cmd = Command::Set { key: "foo".to_string(), value: "bar".into(), resp: rtx };
        let _  = tx.send(cmd).await;
        let res = rrx.await;
        println!("T1 GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (rtx, rrx) = oneshot::channel();
        let cmd = Command::Get { key: "foo".to_string(), resp: rtx};
        let _ = tx2.send(cmd).await;
        let res = rrx.await;
        println!("T2 GOT = {:?}", res);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get {key, resp} => {
                    let res  = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Set { key, value, resp } => {
                    let res  = client.set(&key, value.clone()).await;
                    let _ = resp.send(res);
                }
    
            }
            // println!("GOT = {}", cmd);
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();



    // match rx.recv().await {
    //     Some(message) => {
    //         println!("GOT = {}", message);
    //     }
    //     _ => {}
    // }
}

enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;