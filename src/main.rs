use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process,
};

use webserver::ThreadPool;

fn main() {
    let ip_addr = "127.0.0.1";
    let port: usize = 8080;

    let listener: TcpListener;

    match TcpListener::bind(format!("{ip_addr}:{port}")) {
        Ok(i) => {
            println!("INFO: server created at port: {}", port);
            listener = i
        }
        Err(e) => {
            println!("ERROR: Failed to created server at port: {}. {}", port, e);
            process::exit(1)
        }
    }

    let pool = match ThreadPool::build(4) {
        Ok(p) => p,
        Err(e) => {
            println!("Error in creating thread pool: {}", e);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents: String;

    match fs::read_to_string(filename) {
        Ok(c) => contents = c,
        Err(e) => {
            println!("ERROR: failed to read file: {}. {}", filename, e);
            process::exit(1)
        }
    }

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
