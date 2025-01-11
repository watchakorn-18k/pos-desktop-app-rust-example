use rusqlite::{Connection, Result};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct Product {
    id: String,
    name: String,
    price: u32,
    description: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_foods() -> Vec<Product> {
    vec![
        Product {
            id: Uuid::new_v4().to_string(),
            name: "Apple".to_string(),
            price: 10,
            description: "sss".to_string(),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: "Banana".to_string(),
            price: 20,
            description: "resdsd".to_string(),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: "Orange".to_string(),
            price: 30,
            description: "fsdfdf".to_string(),
        },
    ]
}
#[tauri::command]
fn create_db() -> Result<(), String> {
    let conn = Connection::open("products.db").map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            price INTEGER NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// เพิ่มสินค้าใหม่
#[tauri::command]
fn add_product(name: String, price: u32, description: String) -> Result<(), String> {
    let conn = Connection::open("products.db").map_err(|e| e.to_string())?;

    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO products (id, name, price, description) VALUES (?1, ?2, ?3, ?4)",
        [&id, &name, &price.to_string(), &description],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

//ลบ สินค้า
#[tauri::command]
fn remove_product(id: String) -> Result<(), String> {
    let conn = Connection::open("products.db").map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM products WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

// แก้ไขข้อมูลสินค้า
#[tauri::command]
fn update_product(id: String, name: String, price: u32, description: String) -> Result<(), String> {
    let conn = Connection::open("products.db").map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE products SET name = ?1, price = ?2, description = ?3 WHERE id = ?4",
        [&name, &price.to_string(), &description, &id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// ดึงข้อมูลสินค้าทั้งหมด
#[tauri::command]
fn get_products() -> Result<Vec<Product>, String> {
    let conn = Connection::open("products.db").map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, price, description FROM products")
        .map_err(|e| e.to_string())?;
    let products = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
                description: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for product in products {
        result.push(product.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = create_db();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_foods,
            add_product,
            get_products,
            remove_product,
            update_product
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
