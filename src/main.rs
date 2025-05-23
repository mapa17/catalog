mod catalog;

mod items;
use items::build_catalog;

fn main() {
    let item_catalog = build_catalog();    
    item_catalog.execute_all();
}
