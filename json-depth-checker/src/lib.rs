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
