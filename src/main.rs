mod resp_parser;

use crate::resp_parser::parse_resp;
use std::env::Args;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const DEFAULT_LISTEN_PORT: u16 = 36379;
const DEFAULT_LISTEN_HOST: &str = "127.0.0.1";
const DEFAULT_TARGET_PORT: u16 = 6379;
const DEFAULT_TARGET_HOST: &str = "127.0.0.1";
// const DEFAULT_TARGET_USER: &str = "default";
// const DEFAULT_TARGET_PASSWORD: &str = "";

fn main() {
    let mut args = Args::from(std::env::args());

    let listening_port = args
        .find(|arg| arg.starts_with("--listen-port="))
        .map(|arg| arg.replace("--listen-port=", ""))
        .map(|arg| arg.parse::<u16>().unwrap_or(DEFAULT_LISTEN_PORT))
        .unwrap_or(DEFAULT_LISTEN_PORT);

    let listening_host = args
        .find(|arg| arg.starts_with("--listen-host="))
        .map(|arg| arg.replace("--listen-host=", ""))
        .unwrap_or(DEFAULT_LISTEN_HOST.to_string());

    let target_host = args
        .find(|arg| arg.starts_with("--host="))
        .map(|arg| arg.replace("--host=", ""))
        .unwrap_or(DEFAULT_TARGET_HOST.to_string());
    let target_port = args
        .find(|arg| arg.starts_with("--port="))
        .map(|arg| arg.replace("--port=", ""))
        .map(|arg| arg.parse::<u16>().unwrap_or(DEFAULT_TARGET_PORT))
        .unwrap_or(DEFAULT_TARGET_PORT);

    let listener = TcpListener::bind(format!("{}:{}", listening_host, listening_port)).unwrap();
    println!("Listening on {}:{}...", listening_host, listening_port);
    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        let addr = s.local_addr().unwrap();
        println!(
            "New connection from {}:{}",
            addr.ip().to_string(),
            addr.port()
        );
        handle_connection(&mut s, &target_host, target_port);
    }
}

fn handle_connection(stream: &mut TcpStream, target_host: &String, target_port: u16) {
    println!("Opening new connection to target redis at {}:{}...", target_host, target_port);
    let mut down_stream = TcpStream::connect(format!("{}:{}", target_host, target_port)).unwrap();
    println!("New connection opened at {}:{}...", down_stream.local_addr().unwrap().ip(), down_stream.local_addr().unwrap().port());
    stream.set_read_timeout(Some(std::time::Duration::from_secs(1))).unwrap();
    loop {
        let redis_protocol_request = read_redis_protocol(stream);
        if redis_protocol_request == "" {
            WIP
            continue;
        }
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
            return String::new();
        }
        let i = parse_resp(&line);
        remaining_lines_to_read += i;
        command.push_str(&line);
        if remaining_lines_to_read == 0 {
            return command;
        }
    }
}
