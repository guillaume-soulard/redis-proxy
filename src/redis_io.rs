use crate::resp::get_remaining_lines_to_read;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;

pub struct RedisStream<'a> {
    stream: &'a mut TcpStream,
    buf_reader: &'a mut BufReader<&'a mut TcpStream>,
}

impl<'a> RedisStream<'a> {
    pub fn new(stream: &'a mut TcpStream,
               buf_reader: &'a mut BufReader<&'a mut TcpStream>) -> RedisStream<'a> {
        RedisStream {
            stream,
            buf_reader,
        }
    }

    pub fn send(&mut self, redis_protocol: &String) {
        self.stream
            .write_all(redis_protocol.as_bytes())
            .unwrap_or_else(|e| {
                println!("error writing to down stream {}", e);
            });
    }
    pub fn receive(&mut self) -> Option<String> {
        let mut line = String::new();
        let mut remaining_lines_to_read = 0;
        let mut command = String::new();
        loop {
            line.clear();
            if remaining_lines_to_read > 0 {
                remaining_lines_to_read -= 1;
            }
            let read_bytes = match self.buf_reader.read_line(&mut line) {
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
            let i = get_remaining_lines_to_read(&line);
            remaining_lines_to_read += i;
            command.push_str(&line);
            if remaining_lines_to_read == 0 {
                return Some(command);
            }
        }
    }
}