
use std::collections::HashMap;

use crate::catalog::{ItemTrait, ItemDescription, Priority};

pub struct ItemA {
    number: u32,
    name: &'static str,
    description: &'static str,
    priority: Priority
}

impl ItemTrait for ItemA {
    fn new() -> Box<dyn ItemTrait> {
        return Box::new(ItemA{
            number: 42,
            name: "ItemA",
            description: "Super Interesting Description",
            priority: Priority::LEVEL0
        })
    }

    fn get_description(&self) -> ItemDescription {
        return ItemDescription{
            name: self.name,
            description: self.description,
            priority: self.priority
        }
    }

    fn execute(&self, context: HashMap<String, String>) -> HashMap<String, String> {
        println!("Executing ItemA with {:?}", context);
        let mut c = context.clone();
        c.insert("ItemAResult".to_string(), format!("Great Result: {}", self.number).to_string());
        return c;
    }
}

