use serde_json::{Number, Value, json};

pub(crate) fn decode(encoded_value: &str) -> Value {
    let mut borrow_encoded = encoded_value;
    return decode_bencoded_value(&mut borrow_encoded);
}
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
            *encoded_value = &encoded_value[1..]; // Consume the 'd'
            Value::Array(decode_benencoded_dictionary(encoded_value))
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

fn decode_benencoded_dictionary(_encoded_value: &mut &str) -> Vec<Value> {
    todo!()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let mut encoded_value = "i123e";
        let result = decode_bencoded_value(&mut encoded_value);
        assert_eq!(result, Value::Number(serde_json::Number::from(123)));
        assert_eq!(encoded_value, ""); // Ensure all input was consumed
    }

    #[test]
    fn test_string() {
        let mut encoded_value = "5:hello";
        let result = decode_bencoded_value(&mut encoded_value);
        assert_eq!(result, Value::String("hello".to_string()));
        assert_eq!(encoded_value, ""); // Ensure all input was consumed
    }

    #[test]
    fn test_empty_string() {
        let mut encoded_value = "0:";
        let result = decode_bencoded_value(&mut encoded_value);
        assert_eq!(result, Value::String("".to_string()));
        assert_eq!(encoded_value, ""); // Ensure all input was consumed
    }

    #[test]
    fn test_list() {
        let mut encoded_value = "l5:helloi52ee";
        let result = decode_bencoded_value(&mut encoded_value);
        let expected = json!(["hello", 52]);
        assert_eq!(result, expected);
        assert_eq!(encoded_value, ""); // Ensure all input was consumed
    }

    #[test]
    #[should_panic(expected = "Unhandled encoded value:")]
    fn test_invalid_input() {
        let mut encoded_value = "x123e";
        decode_bencoded_value(&mut encoded_value);
    }
}