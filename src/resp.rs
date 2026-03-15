
const SIMPLE_STRING:char = '+';
const ERROR:char = '-';
const INTEGER:char = ':';
const BULK_STRING:char = '$';
const ARRAY:char = '*';
const NULL:char = '_';
const BOOLEAN:char = '#';
const DOUBLE:char = ',';
const BIG_NUMBER:char = '(';
const BULK_ERROR:char = '!';
const VERBATIM_STRING:char = '=';
const MAP:char = '%';
const ATTRIBUTE:char = '|';
const SET:char = '~';
const PUSH:char = '>';

pub fn get_remaining_lines_to_read(item: &String) -> u32 {
    let resp_type:char = item.chars().next().unwrap();
    match resp_type {
        c @ (ARRAY | SET | PUSH) => {
            let to_parse = item.replace(&String::from(c), "")
                .replace("\r\n", "");
            if to_parse == "" {
                return 0
            }
            to_parse
                .parse::<u32>()
                .unwrap_or_else(|e| panic!("can't parse resp: {} : {}", item, e))
        },
        c @ (BULK_STRING | VERBATIM_STRING) => {
            let to_parse = item.replace(&String::from(c), "")
                .replace("\r\n", "");
            if to_parse == "" {
                return 0
            }
            let parsed = to_parse
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("can't parse resp: {} : {}", item, e));
            if parsed < 0 {
                0
            } else {
                1
            }
        }
        SIMPLE_STRING | ERROR | INTEGER | BOOLEAN | NULL | DOUBLE | BIG_NUMBER | BULK_ERROR => {
            0
        }
        c @ (MAP | ATTRIBUTE) => {
            item.replace(&String::from(c), "")
                .replace("\r\n", "")
                .parse::<u32>()
                .unwrap() * 2
        },
        _ => {
            0
        }
    }
}

pub fn get_command_name(item: &String) -> Option<String> {
    item.split_once("\r\n")
        .map(|(command, _)| {
            command.split_once("\r\n")
                .map(|(command, _)| {
                    command.split_once("\r\n")
                        .map(|(command, _)| command.to_string())
                        .unwrap_or(String::from(""))
                })
                .unwrap_or(String::from(""))
        })
}

pub struct RespBuilder {
    resp_type: char,
    protocol: Vec<String>,
}

impl RespBuilder {
    pub fn new() -> RespBuilder {
        RespBuilder {
            resp_type: ARRAY,
            protocol: Vec::new(),
        }
    }
    pub fn build(&self) -> String {
        self.protocol.join("")
    }
    pub fn append(&mut self, item: &String) {
        if self.resp_type == ARRAY {
            if self.protocol.len() == 0 {
                self.protocol.push("*1\r\n".to_string());
            } else {
                self.protocol[0] = format!("*{}\r\n", self.protocol.len() + 1);
            }
        }
        self.protocol.push(format!("${}\r\n{}\r\n", item.len(), item.clone()));
    }
}
