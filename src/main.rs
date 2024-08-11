use serde_json;
use serde_json::Number;
use serde_json::Value;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &mut &str) -> Value {
    match encoded_value.chars().next() {
        Some('i') => {
            if let Some(end_pos) = encoded_value.find('e') {
                let num_str = &encoded_value[1..end_pos];
                *encoded_value = &encoded_value[end_pos + 1..]; // Consume up to and including 'e'
                Value::Number(
                    num_str
                        .parse::<Number>()
                        .unwrap_or_else(|_| Number::from(0)),
                )
            } else {
                panic!("Invalid integer encoding: {}", encoded_value);
            }
        }
        Some('d') if encoded_value.ends_with('e') => {
            // dictionary
            panic!("dictionary parsing not supported!");
        }
        Some('l') if encoded_value.ends_with('e') => {
            // list
            *encoded_value = &encoded_value[1..]; // Consume the 'l'
            Value::Array(decode_benencoded_list(encoded_value))
        }
        Some(c) if c.is_digit(10) => {
            // string
            if let Some((len_str, rest)) = encoded_value.split_once(':') {
                if let Ok(len) = len_str.parse::<usize>() {
                    let string_value = &rest[..len];
                    *encoded_value = &rest[len..]; // Consume the string part
                    return Value::String(string_value.to_string());
                } else {
                    panic!("Invalid length in encoded value: {}", encoded_value);
                }
            } else {
                panic!("Incorrectly encoded string value: {}", encoded_value);
            }
        }
        _ => {
            if encoded_value.is_empty() {
                Value::Null
            } else {
                panic!("Unhandled encoded value: {}", encoded_value);
            }
        }
    }
}

fn decode_benencoded_list(encoded_value: &mut &str) -> Vec<Value> {
    let mut elements = Vec::new();

    while !encoded_value.starts_with('e') {
        if encoded_value.is_empty() {
            panic!("Unexpected end of encoded value while parsing list");
        }

        // Parse the next element recursively
        let element = decode_bencoded_value(encoded_value);
        // println!("Current resolved encoded_value: {}", element);

        elements.push(element);

        // println!("Remaining encoded_value: {}", encoded_value);
    }

    // Consume the 'e' at the end of the list
    *encoded_value = &encoded_value[1..];

    elements
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let mut encoded_value = args[2].as_str();
        let decoded_value = decode_bencoded_value(&mut encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
