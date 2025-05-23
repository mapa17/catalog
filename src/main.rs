mod catalog;
use catalog::Catalog;
use crate::catalog::ItemTrait;
mod items;
use items::ItemA; 

fn main() {
    let mut catalog = Catalog::new();
    catalog.register_item(ItemA::new());
    catalog.execute_all();
}
