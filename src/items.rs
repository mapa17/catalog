mod item_a;
pub use item_a::ItemA;

mod item_b;
pub use item_b::ItemB;

mod item_c;
pub use item_c::ItemC;

use crate::catalog::{
    Catalog,
    ItemTrait
};

pub fn build_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    catalog.register_item(ItemA::new());
    catalog.register_item(ItemB::new());
    catalog.register_item(ItemC::new());
    return catalog;
}