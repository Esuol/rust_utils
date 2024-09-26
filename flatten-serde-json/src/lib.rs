#![doc = include_str!("../README.md")]

use serde_json::{Map, Value};

pub fn flatten(json: &Map<String, Value>) -> Map<String, Value> {
    let mut obj = Map::new();
    let mut all_entries = vec![];
    insert_object(&mut obj, None, json, &mut all_entries);
}

fn insert_array<'a>(
    base_json: &mut Map<String, Value>,
    base_key: &str,
    array: &'a Vec<Value>,
    all_entries: &mut Vec<(String, &'a Value)>,
) {
    for value in array {
        if let Some(object) = value.as_object() {
            insert_object(base_json, Some(base_key), object, all_entries);
        } else if let Some(sub_array) = value.as_array() {
            insert_array(base_json, base_key, sub_array, all_entries);
        } else {
            insert_value(base_json, base_key, value.clone(), true);
        }
    }
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
