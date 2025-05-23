use std::collections::HashMap;
use std::sync::Arc;

pub struct ItemDescription {
    pub name: &'static str,
    pub description: &'static str,
}

pub trait ItemTrait {
    fn new() -> Box<dyn ItemTrait>
    where Self: Sized;
    fn get_description(&self) -> ItemDescription;
    fn execute(&self, context: HashMap<String, String>) -> HashMap<String, String>;
}
pub struct Catalog {
    items: HashMap<String, Box<dyn ItemTrait>>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            items: HashMap::new(),
        }
    }
    
    pub fn register_item(&mut self, item: Box<dyn ItemTrait>) {
        let description = item.get_description();
        println!("Register {} ...", description.name);
        self.items.insert(description.name.to_string(), item);
    }
    
    pub fn execute_all(&self) {
        println!("Executing all registered items {}...", self.items.len());

        // TODO: Execute items in parallel using Rayon

        for (name, item) in &self.items {
            println!("Executing item: {}", name);

            let mut context = HashMap::new();
            context.insert(String::from("name"), name.clone());
            let response = item.execute(context);

            println!("Result {:?}", response);
            
        }
    }
}