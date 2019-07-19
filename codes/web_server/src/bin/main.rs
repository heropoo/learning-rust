use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;
use std::thread::Thread;
use web_server::ThreadPool;

//extern crate httparse;

fn main() {
    let host = "127.0.0.1:7878";

    let listener = TcpListener::bind(host).unwrap();

    println!("Listening on http://{}", host);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
//        pool.execute(|| {
//            handle_connection(stream);
//        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    println!("Reqeust: {}", String::from_utf8_lossy(&buffer[..]));

//    let request = String::from_utf8_lossy(&buffer[..]);

//    println!("Reqeust Method: {}", get_request_method(&request));
//    println!("Reqeust Method: {}", Method::from_bytes(&request));
//    println!("Reqeust URL: {}", get_request_uri(&request));
//    println!("Reqeust URL: {}", get_request_uri(&request));


//    let mut headers = [httparse::EMPTY_HEADER; 16];
//    let mut req = httparse::Request::new(&mut headers);
//    let res = req.parse(request.as_bytes()).unwrap();
//    if res.is_complete() {
//        println!("Request URI: {:?}", req.path);
//        println!("Request Method: {:?}", req.method);
//        println!("Request Version: {:?}", req.version);
//        println!("Request Headers: {:?}", req.headers);
//    }

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

//fn get_request_method(s: &str) -> &str{
//    let bytes = s.as_bytes();
//    for (i, &item) in bytes.iter().enumerate(){
//        if item == b' ' {
//            return &s[0..i];
//        }
//    }
//    return "";
//}
//
//fn get_request_uri(s: &str) -> &str{
//
//    let mut start = 0;
//    let mut end = 0;
//    let mut count = 0;
//
//    let bytes = s.as_bytes();
//    for (i, &item) in bytes.iter().enumerate(){
//        if item == b' ' {
//            //return &s[0..i];
//            if count > 0 {
//                end = i;
//                break;
//            }else{
//                start = i;
//                count += 1;
//            }
//        }
//    }
//
//    return &s[start..end];
//}
