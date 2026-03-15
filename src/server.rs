use std::collections::HashMap;
use crate::redis_io::RedisStream;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use crate::resp::{get_command_name, RespBuilder};

pub fn start_server(listening_host: String,
                    listening_port: u16,
                    target_host: String,
                    target_port: u16) {
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
    let mut m:HashMap<String, Box<dyn Fn(&mut RedisStream, &String)>> = HashMap::new();
    m.insert(String::from("role"), Box::new(|up_stream_client, _| {
        let mut role_response = RespBuilder::new();
        role_response.append(&String::from("proxy"));
        up_stream_client.send(&role_response.build());
    }));
    loop {
        {
            match up_stream_client.receive() {
                Some(r) => {
                    match get_command_name(&r) {
                        Some(command_name) => {
                            let command = m.get(&command_name.to_lowercase());
                            match command {
                                Some(cmd) => {
                                    cmd(up_stream_client, &r);
                                    continue;
                                },
                                None => (),
                            }
                        }
                        None => continue,
                    }
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
