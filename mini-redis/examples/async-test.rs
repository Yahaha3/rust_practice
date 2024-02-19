use tokio::{self, join};
use tokio::{time};

#[tokio::main]
async fn main() {

    join!(
        dance(), 
    async{
        sing().await;
    },
    async{
        let s = say().await;
        println!("{}", s);
    });
}

async fn get_song() -> String {
    println!("get song");
    String::from("make song")
}

async fn sing() {
    join!(get_song(), 
    async {
        time::sleep(time::Duration::from_secs(3)).await;
    },
    async{
        println!("sing song");
    });
}

async fn dance() {
    println!("dance");
}

async fn say() -> String{
    String::from("hello world")
}