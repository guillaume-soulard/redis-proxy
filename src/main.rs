mod resp_parser;
mod redis_io;

use std::env::Args;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use crate::redis_io::RedisStream;

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
        let mut cloned_stream = stream.unwrap().try_clone().unwrap();
        let target_host_clone = target_host.clone();
        spawn(move || {
            let addr = cloned_stream.local_addr().unwrap();
            println!(
                "New connection from {}:{}",
                addr.ip().to_string(),
                addr.port()
            );
            let mut up_stream_binding = cloned_stream.try_clone().unwrap();
            let mut up_stream_buf_reader = BufReader::new(&mut up_stream_binding);
            let mut up_stream_client = RedisStream::new(&mut cloned_stream, &mut up_stream_buf_reader);
            println!("Opening new connection to target");
            let mut down_stream = TcpStream::connect(format!("{}:{}", target_host_clone, target_port)).unwrap();
            println!("New connection opened");
            let mut down_stream_binding = down_stream.try_clone().unwrap();
            let mut down_stream_buf_reader = BufReader::new(&mut down_stream_binding);
            let mut down_stream_client = RedisStream::new(&mut down_stream, &mut down_stream_buf_reader);
            handle_connection(&mut up_stream_client, &mut down_stream_client);
            println!(
                "Connection closed by client : {}:{}",
                addr.ip().to_string(),
                addr.port()
            );
        });
    }
}

fn handle_connection(up_stream_client: &mut RedisStream,
                     down_stream_client: &mut RedisStream) {
    loop {
        {
            match up_stream_client.receive() {
                Some(r) => {
                    if r == "" {
                        continue;
                    } else {
                        down_stream_client.send(&r);
                    }
                },
                None => {
                    break;
                }
            }
        }
        {
            let redis_protocol_response = down_stream_client.receive();
            up_stream_client.send(&redis_protocol_response.unwrap_or_else(|| String::from("$-1\r\n")));
        }
    }
}
