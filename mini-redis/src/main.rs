use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
// use tokio::task::yield_now;
// use std::rc::Rc;
use bytes::Bytes;
use core::num;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
    // tokio::spawn(async {
    //     {
    //         let rc = Rc::new("hello");
    //         println!("{}", rc);
    //     }
    //     yield_now().await;
    // });
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap()
    }
}

// fn new_shared_db(num_shareds: usize) -> ShardDb {
//     let mut db = Vec::with_capacity(num_shareds);
//     for _ in 0..num_shareds {
//         db.push(Mutex::new(HashMap::new));
//     }
//     Arc::new(db)
// }