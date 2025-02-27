use serde_json::Value;

pub fn should_flatten_from_unchecked_slice(json: &[u8]) -> bool {
    if json.is_empty() {
        return false;
    }

    if json[0] == b'{' {
        return true;
    } else if json[0] != b'[' {
        return false;
    }

    let mut skip_next = false;
    let mut in_string = false;

    for byte in json.iter().skip(1) {
        match byte {
            // handle the backlash.
            _ if skip_next => skip_next = false,
            b'\\' => skip_next = true,

            // handle the strings.
            byte if in_string => {
                if *byte == b'"' {
                    in_string = false;
                }
            }
            b'"' => in_string = true,

            // handle the arrays.
            b'[' => return true,
            // since we know the json is valid we don't need to ensure the
            // array is correctly closed

            // handle the objects.
            b'{' => return true,

            // ignore everything else
            _ => (),
        }
    }

    false
}

pub fn should_flatten_from_value(json: &Value) -> bool {
    match json {
        Value::Object(..) => true,
        Value::Array(array) => array
            .iter()
            .any(|value| value.is_array() || value.is_object()),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::*;

    use super::*;

    #[test]
    fn test_shouldnt_flatteb() {
        let shouldnt_flatten = vec![
            json!(null),
            json!(true),
            json!(false),
            json!("a superb string"),
            json!("a string escaping other \"string\""),
            json!([null, true, false]),
            json!(["hello", "world", "!"]),
            json!(["a \"string\" escaping 'an other'", "\"[\"", "\"{\""]),
        ];

        for value in shouldnt_flatten {
            assert!(!should_flatten_from_value(&value));
            let value = serde_json::to_vec(&value).unwrap();
            assert!(!should_flatten_from_unchecked_slice(&value));
        }
    }

    #[test]
    fn test_should_flatten() {
        let should_flatten = vec![
            json!({}),
            json!({ "hello": "world" }),
            json!(["hello", ["world"]]),
            json!([true, true, true, true, true, true, true, true, true, {}]),
        ];
        for value in should_flatten {
            assert!(should_flatten_from_value(&value));
            let value = serde_json::to_vec(&value).unwrap();
            assert!(should_flatten_from_unchecked_slice(&value));
        }
    }
}
