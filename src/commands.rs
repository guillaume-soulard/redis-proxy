use std::collections::HashMap;
use crate::redis_io::RedisStream;
use crate::resp::RespBuilder;

pub fn init_commands(m: &mut HashMap<String, Box<dyn Fn(&mut RedisStream, &String)>>) {
    role_command(m);
}

fn role_command(m: &mut HashMap<String, Box<dyn Fn(&mut RedisStream, &String)>>) {
    m.insert(String::from("role"), Box::new(|up_stream_client, _| {
        let mut role_response = RespBuilder::new();
        role_response.append(&String::from("proxy"));
        up_stream_client.send(&role_response.build());
    }));
}
