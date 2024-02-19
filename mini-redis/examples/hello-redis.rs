use mini_redis::{client, Result};
#[tokio::main]
async fn main() -> Result<()>{
    // println!("Hello, world!");
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("从服务器中获取到的结果是{:?}", result);

    Ok(())
}
