use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{env, fs, thread};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let threads = if let Some(x) = env::args().nth(1) {
        x.parse().unwrap()
    } else {
        4 as usize
    };

    let pool = ThreadPool::new(threads).unwrap_or_else(|e| panic!("{}", e.0));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string("pages/".to_string() + filename).unwrap();
    let content_len = format!("Content-Length: {}", contents.len());
    let response = format!("{}\r\n{}\r\n\r\n{}", status_line, content_len, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
