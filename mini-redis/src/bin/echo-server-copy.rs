use std::vec;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]

async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        // let (mut rd, mut wr) = io::split(socket);
        let _ = tokio::spawn(async move {
            // let (mut rd, mut wr) = socket.split();
            // if io::copy(&mut rd, &mut wr).await.is_err() {
            //     eprintln!("failed to copy");
            // }
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    },
                    Err(_) => return
                }
            }
        });
    }

    // Ok(())
}