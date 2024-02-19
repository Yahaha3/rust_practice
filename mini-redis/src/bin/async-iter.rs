use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(&[1,2,3]);

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }

    // let vv = vec![1,23,3];

    // let mut i = 0;
    // while let Some(v) = vv.get(i) {
    //     i += 1;
    //     println!("GOT = {:?}", v);
    // }
}