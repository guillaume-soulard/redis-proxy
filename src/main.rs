mod resp_parser;
mod redis_io;
mod server;

use std::env::Args;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use crate::redis_io::RedisStream;
use crate::server::start_server;

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
    let target_topology = args
        .find(|arg| arg.starts_with("--topology="))
        .map(|arg| arg.replace("--topology=", ""))
        .unwrap_or("standalone".to_string());

    start_server(listening_host, listening_port, target_host, target_port, target_topology);
}
