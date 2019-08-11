use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use blog::controllers::index;
use blog::controllers::login;

// 躲了一辈子的雨，雨会不会很难过啊……
pub fn run(host: &str) {
    let listener = TcpListener::bind(host).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let login = b"GET /login HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    println!("Reqeust: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(get) {
        // ("HTTP/1.1 200 OK\r\n\r\n", "tpls/hello.html")
        // todo
        index::index()
    } else if buffer.starts_with(login) {
        login::login()
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "tpls/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "tpls/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
