use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    match ThreadPool::build(12) {
        Ok(pool) => {
            for stream in listener.incoming() {
                pool.execute(|| {
                    handle_connection(stream.unwrap());
                });
            }
        }
        Err(err) => println!("{}", err),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Received: {}", request_line);
    let (status_line, filename, path) = match &request_line[..] {
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html", "GET /sleep".to_string())
        }
        request => {
            let split = request.split(" ");
            let path = split.take(2).collect::<String>();
            ("HTTP/1.1 200 OK", "hello.html", path)
        }
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("Served: {}", path);
    stream.write_all(response.as_bytes()).unwrap();
}
