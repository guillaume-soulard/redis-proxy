mod resp;
mod redis_io;
mod server;

use crate::server::start_server;
use std::env::Args;

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

    start_server(listening_host, listening_port, target_host, target_port);
}
