mod repositories;
mod services;

use repositories::products_repo::Product;
use services::products_service;
use tauri::command;

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn get_foods() -> Vec<Product> {
    vec![
        Product {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Apple".to_string(),
            price: 10,
            description: "Fresh apple".to_string(),
        },
        Product {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Banana".to_string(),
            price: 20,
            description: "Ripe banana".to_string(),
        },
    ]
}

#[command]
fn initialize_db() -> Result<(), String> {
    products_service::initialize_database().map_err(|e| e.to_string())
}

#[command]
fn add_product(name: String, price: u32, description: String) -> Result<(), String> {
    products_service::add_product(name, price, description).map_err(|e| e.to_string())
}

#[command]
fn remove_product(id: String) -> Result<(), String> {
    products_service::remove_product(id).map_err(|e| e.to_string())
}

#[command]
fn update_product(id: String, name: String, price: u32, description: String) -> Result<(), String> {
    products_service::update_product(id, name, price, description).map_err(|e| e.to_string())
}

#[command]
fn get_products() -> Result<Vec<Product>, String> {
    products_service::get_all_products().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = products_service::initialize_database();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_foods,
            initialize_db,
            add_product,
            remove_product,
            update_product,
            get_products
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
