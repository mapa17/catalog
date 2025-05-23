use std::sync::Arc;

use super::{ItemTrait, WrappedItem};

struct ItemB {
    name: &'static str,
    duration: f32,
}

impl ItemTrait for ItemB {
    fn get_name(&self) -> String {
        return self.name.to_string();
    }

    fn execute(&self, text: String) -> String {
        return format!("Executing ItemB with {0}", text);
    }
}

inventory::submit! {
    WrappedItem {
        item: Arc::new(ItemB { 
            name: "Mark", 
            duration: 3.1415 
        })
    }
}