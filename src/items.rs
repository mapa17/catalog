mod item_a;
pub use item_a::ItemA;

use crate::catalog::{
    Catalog,
    ItemTrait
};

pub fn build_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    catalog.register_item(ItemA::new());
    return catalog;
}