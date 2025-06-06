/*
Catalog - Keep track of multiple items and execute them by priority in parallel

Catalog is a struct that can be used to register multiple items that each has to implement
the `ItemTrait` Trait.

A recommended structure looks similar to

src
 - items
    - itemA.rs
    - itemB.rs
    items.rs
    catalog.rs

Where in `items.rs` a catalog struct is instantiated and all the item instances are registered.

*/

use std::collections::HashMap;
use std::thread;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Priority {
    LEVEL0,
    LEVEL1,
    LEVEL2
}

impl Priority {
    pub const fn get_levels() -> &'static [Priority] {
        &[Priority::LEVEL0, Priority::LEVEL1, Priority::LEVEL2]
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct ItemDescription {
    pub name: &'static str,
    pub description: &'static str,
    pub priority: Priority,
}

pub trait ItemTrait: Send + Sync{
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
        println!("Register {} for level {:?}...", description.name, description.priority);
        self.items.insert(description.name.to_string(), item);
    }
    
    pub fn execute(&self, context: &mut HashMap<String, String>){        
        println!("Executing all registered items {}...", self.items.len());
        for current_level in Priority::get_levels() {
            
            // Only select items for current level            
            let items_to_execute: Vec<(&String, &Box<dyn ItemTrait>)> = self
                .items
                .iter()
                .filter(|(_, item)| item.get_description().priority == *current_level)
                .collect();

            if items_to_execute.len() > 0 {
                println!("Executing {} items with level {:?} and context {:?}", items_to_execute.len(), *current_level, context);
                // Execute every item in its own thead. Collect and join all partial results into a new context.
                let new_context: HashMap<String, String> = thread::scope(|s| {
                    items_to_execute
                        .iter()
                        .map(|(_, item)| {
                            let thread_context = context.clone();
                            s.spawn(|| item.execute(thread_context))})
                        .map(|h| h.join().unwrap())
                        .flatten()
                        .collect()
                });

                context.extend(new_context);
            }
        }
        println!("Final result {context:?}");
    }
}