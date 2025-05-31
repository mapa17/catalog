use std::collections::HashMap;

use crate::catalog::{ItemTrait, ItemDescription, Priority};

pub struct ItemC {
    name: &'static str,
    description: &'static str,
    priority: Priority
}

impl ItemTrait for ItemC {
    fn new() -> Box<dyn ItemTrait> {
        return Box::new(ItemC{
            name: "ItemC",
            description: "This is ItemC at L1",
            priority: Priority::LEVEL1
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
        println!("Executing ItemC with {:?}", context);
        let mut c = context.clone();
        c.insert("ItemBResult".to_string(), format!("Result from ItemC: 24").to_string());
        return c;
    }
}

