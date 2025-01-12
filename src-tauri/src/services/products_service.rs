use crate::repositories::products_repo::{self, Product};
use rusqlite::Result;

pub fn initialize_database() -> Result<()> {
    products_repo::create_db()
}

pub fn add_product(name: String, price: u32, description: String) -> Result<()> {
    products_repo::add_product_to_db(name, price, description)
}

pub fn remove_product(id: String) -> Result<()> {
    products_repo::remove_product_from_db(id)
}

pub fn update_product(id: String, name: String, price: u32, description: String) -> Result<()> {
    products_repo::update_product_in_db(id, name, price, description)
}

pub fn get_all_products() -> Result<Vec<Product>> {
    products_repo::fetch_all_products()
}
