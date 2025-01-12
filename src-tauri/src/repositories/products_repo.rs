use rusqlite::{Connection, Result};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub description: String,
}

pub fn create_db() -> Result<()> {
    let conn = Connection::open("products.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            price INTEGER NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn add_product_to_db(name: String, price: u32, description: String) -> Result<()> {
    let conn = Connection::open("products.db")?;
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO products (id, name, price, description) VALUES (?1, ?2, ?3, ?4)",
        [&id, &name, &price.to_string(), &description],
    )?;

    Ok(())
}

pub fn remove_product_from_db(id: String) -> Result<()> {
    let conn = Connection::open("products.db")?;

    conn.execute("DELETE FROM products WHERE id = ?1", [&id])?;

    Ok(())
}

pub fn update_product_in_db(
    id: String,
    name: String,
    price: u32,
    description: String,
) -> Result<()> {
    let conn = Connection::open("products.db")?;

    conn.execute(
        "UPDATE products SET name = ?1, price = ?2, description = ?3 WHERE id = ?4",
        [&name, &price.to_string(), &description, &id],
    )?;

    Ok(())
}

pub fn fetch_all_products() -> Result<Vec<Product>> {
    let conn = Connection::open("products.db")?;
    let mut stmt = conn.prepare("SELECT id, name, price, description FROM products")?;
    let products = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    let mut result = Vec::new();
    for product in products {
        result.push(product?);
    }

    Ok(result)
}
