use std::io::stdin;

use flatten_serde_json::flatten;
use serde_json::{Map, Value};

fn main() {
    let json: Map<String, Value> = serde_json::from_reader(stdin()).unwrap();
    let flat = flatten(&json);
    println!("{}", serde_json::to_string(&flat).unwrap());
}
