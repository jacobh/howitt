use std::borrow::Borrow;

#[allow(dead_code)]
pub fn prettyprintln(value: impl Borrow<serde_json::Value>) {
    println!("{}", serde_json::to_string_pretty(value.borrow()).unwrap());
}
