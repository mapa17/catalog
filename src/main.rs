mod catalog;

mod items;
use std::collections::HashMap;

use items::build_catalog;

fn main() {
    let mut results: HashMap<String, String> = HashMap::new();
    results.insert("InitialKey".to_string(), "SecretValue".to_string());
    let item_catalog = build_catalog();    
    item_catalog.execute(&mut results);
}
