use serde_json::{json, Number, Value};

pub(crate) fn decode(encoded_value: &str) -> Value {
    return decode_bencoded_value(&mut encoded_value.as_bytes().to_vec());
}
fn decode_bencoded_value(encoded_value: &mut Vec<u8>) -> Value {
    match encoded_value.first() {
        Some(b'i') => {
            if let Some(end_pos) = encoded_value.iter().position(|&c| c == b'e') {
                let num_slice = &encoded_value[1..end_pos];
                let mut number = 0i64;
                let mut negative = false;

                for &byte in num_slice {
                    match byte {
                        b'-' if number == 0 => negative = true,
                        b'0'..=b'9' => {
                            number = number * 10 + (byte - b'0') as i64;
                        }
                        _ => panic!("Invalid character in integer encoding"),
                    }
                }

                if negative {
                    number = -number;
                }

                *encoded_value = encoded_value[end_pos + 1..].to_vec(); // Consume up to and including 'e'
                Value::Number(Number::from(number))
            } else {
                panic!("Invalid integer encoding: {:?}", encoded_value);
            }
        }
        Some(b'd')  => {
            // dictionary
            *encoded_value = encoded_value[1..].to_vec(); // Consume the 'd'
            let mut map = serde_json::Map::new();

            while encoded_value.first() != Some(&b'e') {
                if encoded_value.is_empty() {
                    panic!("Unexpected end of encoded value while parsing dictionary");
                }
                let k = decode_bencoded_value(encoded_value);
                if let Value::String(key) = k {
                    let v = decode_bencoded_value(encoded_value);
                    map.insert(key, v);
                } else {
                    panic!("JSON only supports keys of type string")
                }
            }
            *encoded_value = encoded_value[1..].to_vec(); // consume ending 'e'

           Value::Object(map)
        }
        Some(b'l') => {
            // list
            *encoded_value = encoded_value[1..].to_vec(); // Consume the 'l'
            let mut elements = Vec::new();

            while encoded_value.first() != Some(&b'e') {
                if encoded_value.is_empty() {
                    panic!("Unexpected end of encoded value while parsing list");
                }
                let element = decode_bencoded_value(encoded_value);
                elements.push(element);
            }
            *encoded_value = encoded_value[1..].to_vec();
            Value::Array(elements)
        }
        Some(c) if (*c as char).is_digit(10) => {
            // string
            if let Some(colon_pos) = encoded_value.iter().position(|&c| c == b':') {
                let len_slice = &encoded_value[..colon_pos];
                let mut length = 0usize;

                for &byte in len_slice {
                    match byte {
                        b'0'..=b'9' => {
                            length = length * 10 + (byte - b'0') as usize;
                        }
                        _ => panic!("Invalid character in string length"),
                    }
                }

                let rest = &encoded_value[colon_pos + 1..];
                if rest.len() < length {
                    panic!("Invalid length in string value: {:?}", encoded_value);
                }
                let string_value = String::from_utf8(rest[..length].to_vec())
                    .expect("Invalid UTF-8 in string value");
                *encoded_value = rest[length..].to_vec(); // consume string part
                return Value::String(string_value);
            } else {
                panic!("Incorrectly encoded string value: {:?}", encoded_value);
            }
        }
        _ => {
            if encoded_value.is_empty() {
                Value::Null
            } else {
                panic!("Unhandled encoded value: {:?}", encoded_value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::AssertUnwindSafe;

    #[test]
    fn test_string() {
        let mut encoded_value = b"5:hello".to_vec();
        let result = decode_bencoded_value(&mut encoded_value);
        assert_eq!(result, Value::String("hello".to_string()));
        assert!(encoded_value.is_empty()); // Ensure all input was consumed
    }

    #[test]
    fn test_empty_string() {
        let mut encoded_value = b"0:".to_vec();
        let result = decode_bencoded_value(&mut encoded_value);
        assert_eq!(result, Value::String("".to_string()));
        assert!(encoded_value.is_empty()); // Ensure all input was consumed
    }

    #[test]
    fn test_list() {
        let mut encoded_value = b"l5:helloi52ee".to_vec();
        let result = decode_bencoded_value(&mut encoded_value);
        let expected = json!(["hello", 52]);
        assert_eq!(result, expected);
        assert!(encoded_value.is_empty()); // Ensure all input was consumed
    }

    #[test]
    fn test_dictionary() {
        let mut encoded_value = b"d3:foo3:bar5:helloi52ee".to_vec();
        let result = decode_bencoded_value(&mut encoded_value);
        let expected = json!({"foo":"bar","hello":52});
        assert_eq!(result, expected);
        assert!(encoded_value.is_empty()); // Ensure all input was consumed
    }

    #[test]
    #[should_panic(expected = "Unhandled encoded value:")]
    fn test_invalid_input() {
        let mut encoded_value = b"x123e".to_vec();
        decode_bencoded_value(&mut encoded_value);
    }

    // #[test]
    // fn test_non_utf8_string() {
    //     let mut encoded_value = vec![b'1', b':', 0xff];
    //     let result = decode_bencoded_value(&mut encoded_value);
    //     assert_eq!(result, Value::String(String::from_utf8_lossy(&[0xff]).into_owned()));
    //     assert!(encoded_value.is_empty()); // Ensure all input was consumed
    // }
    //
    // #[test]
    // fn test_non_utf8_string_in_list() {
    //     let mut encoded_value = vec![b'l', b'1', b':', 0xff, b'e'];
    //     let result = decode_bencoded_value(&mut encoded_value);
    //     let expected = json!([String::from_utf8_lossy(&[0xff]).into_owned()]);
    //     assert_eq!(result, expected);
    //     assert!(encoded_value.is_empty()); // Ensure all input was consumed
    // }

    #[test]
    fn test_incomplete_integer() {
        let mut encoded_value = b"i123".to_vec();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| decode_bencoded_value(&mut encoded_value)));
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_string() {
        let mut encoded_value = b"4:hel".to_vec();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| decode_bencoded_value(&mut encoded_value)));
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_list() {
        let mut encoded_value = b"l5:hello".to_vec();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| decode_bencoded_value(&mut encoded_value)));
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_dictionary() {
        let mut encoded_value = b"d3:foo3:bar".to_vec();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| decode_bencoded_value(&mut encoded_value)));
        assert!(result.is_err());
    }
}