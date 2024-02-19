use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // read_end().await
    write_all().await
}

async fn read() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0;10];
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

async fn read_end () -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", &buffer);
    Ok(())
}

async fn write() -> io::Result<()> {
    let mut file = File::create("bar.txt").await?;
    let n = file.write(b"ABCDEFGHIJKLMN").await?;
    println!("Wrote the first {} bytes", n);
    Ok(())
}

async fn write_all() -> io::Result<()> {
    let mut ss:&[u8] = b"ABCDEFGHIJKLMNOPQ";
    let mut file = File::create("bar.txt").await?;
    // file.write_all(b"ABCDEFGHIJKLMN").await?;
    io::copy(&mut ss, &mut file).await?;
    Ok(())
}