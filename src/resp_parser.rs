
pub fn parse_resp(item: &String) -> u32 {
    let resp_type = item.chars().next().unwrap();
    match resp_type {
        c @ ('*' | '~' | '>') => {
            let to_parse = item.replace(&String::from(c), "")
                .replace("\r\n", "");
            if to_parse == "" {
                return 0
            }
            to_parse
                .parse::<u32>()
                .unwrap_or_else(|e| panic!("can't parse resp: {} : {}", item, e))
        },
        c @ '$' => {
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
        '+' | '-' | ':' | '&' | '_' | '#' | ',' | '(' | '!' | '=' => {
            0
        }
        c @ ('%' | '|') => {
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
