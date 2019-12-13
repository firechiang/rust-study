use std::io::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::Thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9909").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 开启一个线程处理连接
        thread::spawn(|| {
            // 处理连接
            handle_connection(stream);
            println!("有客户端连上来了");
        });
    }
}

// 处理连接
fn handle_connection(mut stream: TcpStream) {
    // 定义一个长度是512的数组
    let mut buffer = [0; 512];
    // 获取客户端发来的信息
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else {
        ("HTTP/1.1 200 NOT FOUND\r\n\r\n", "404.html")
    };
    // 返回信息
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    // 清空返回信息
    stream.flush().unwrap();
}














