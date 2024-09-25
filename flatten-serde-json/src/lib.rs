#![doc = include_str!("../README.md")]

use serde_json::{Map, Value};

pub fn flatten(json: &Map<String, Value>) -> Map<String, Value> {
    let mut obj = Map::new();
    let mut all_entries = vec![];
    insert_object(&mut obj, None, json, &mut all_entries);
}

fn insert_value(
    base_json: &mut Map<String, Value>,
    key: &str,
    to_insert: Value,
    came_from_array: bool,
) {
    debug_assert!(!to_insert.is_object());
    debug_assert!(!to_insert.is_array());

    if let Some(value) = base_json.get_mut(key) {
        if let Some(array) = value.as_array_mut() {
            array.push(to_insert);
        } else {
            let value = std::mem::take(value);
            base_json[key] = Value::Array(vec![value, to_insert]);
        }
    } else if came_from_array {
        base_json.insert(key.to_string(), Value::Array(vec![to_insert]));
    } else {
        base_json.insert(key.to_string(), to_insert);
    }
}
