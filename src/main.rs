mod resp_parser;

use crate::resp_parser::parse_resp;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const PORT:u16 = 6380;
const HOST:&str = "127.0.0.1";

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT)).unwrap();
    println!("Listening on {}:{}...", HOST, PORT);
    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        let addr = s.local_addr().unwrap();
        println!("New connection from {}:{}", addr.ip().to_string(), addr.port());
        handle_connection(&mut s);
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut down_stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    loop {
        let redis_protocol_request = read_redis_protocol(stream);
        send_to(&mut down_stream, &redis_protocol_request);
        let redis_protocol_response = read_redis_protocol(&mut down_stream);
        send_to(stream, &redis_protocol_response);
    }
}

fn send_to(stream: &mut TcpStream, redis_protocol: &String) {
    stream
        .write_all(redis_protocol.as_bytes())
        .unwrap_or_else(|e| {
            println!("error writing to down stream {}", e);
        });
}

fn read_redis_protocol(stream: &mut TcpStream) -> String {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    let mut remaining_lines_to_read = 0;
    let mut command = String::new();
    loop {
        line.clear();
        if remaining_lines_to_read > 0 {
            remaining_lines_to_read -= 1;
        }
        let read_bytes = buf_reader.read_line(&mut line).unwrap_or_else(|e| {
            println!("error reading line {}", e);
            0
        });
        if read_bytes == 0 {
            return String::new()
        }
        let i = parse_resp(&line);
        remaining_lines_to_read += i;
        command.push_str(&line);
        if remaining_lines_to_read == 0 {
            return command
        }
    }
}
