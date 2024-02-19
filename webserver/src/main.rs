use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use webserver::ThreadPool;

fn main() {
    // println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:10258").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("over");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    // .lines()
    // .map(|result| result.unwrap())
    // .take_while(|line| line.is_empty())
    // .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // let (status, file) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    let (status, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(15));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();
    let response = format!("{status}/r/nContent-Length: {length}\r\n\r\n{contents}");
    let _ = stream.write_all(response.as_bytes()).unwrap();

    // println!("Requset: {:#?}", http_request);
}
