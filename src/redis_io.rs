use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;
use crate::resp_parser::parse_resp;

pub struct RedisStream<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> RedisStream<'a> {
    pub fn new(stream: &'a mut TcpStream) -> RedisStream<'a> {
        RedisStream { stream }
    }

    pub fn send(&mut self, redis_protocol: &String) {
        self.stream
            .write_all(redis_protocol.as_bytes())
            .unwrap_or_else(|e| {
                println!("error writing to down stream {}", e);
            });
    }
    pub fn receive(&mut self) -> Option<String> {
        let mut buf_reader = BufReader::new(self.stream.try_clone().unwrap());
        let mut line = String::new();
        let mut remaining_lines_to_read = 0;
        let mut command = String::new();
        loop {
            line.clear();
            if remaining_lines_to_read > 0 {
                remaining_lines_to_read -= 1;
            }
            let read_bytes = match buf_reader.read_line(&mut line) {
                Ok(bytes) => {
                    if bytes == 0 {
                        return None;
                    }
                    Some(bytes)
                },
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    Some(0)
                }
                Err(e) => {
                    if e.kind() == ErrorKind::ConnectionReset ||
                        e.kind() == ErrorKind::BrokenPipe {
                        None
                    } else {
                        println!("Error reading from stream: {}", e);
                        None
                    }
                }
            };
            if read_bytes.is_none() {
                return None;
            }
            if read_bytes.unwrap() == 0 {
                return Some(String::new());
            }
            let i = parse_resp(&line);
            remaining_lines_to_read += i;
            command.push_str(&line);
            if remaining_lines_to_read == 0 {
                return Some(command);
            }
        }
    }
}