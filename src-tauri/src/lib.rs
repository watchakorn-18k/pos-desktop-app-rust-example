use serde::Serialize;

#[derive(Serialize)]
struct Product {
    name: String,
    price: u32,
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
            name: "Apple".to_string(),
            price: 10,
        },
        Product {
            name: "Banana".to_string(),
            price: 20,
        },
        Product {
            name: "Orange".to_string(),
            price: 30,
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_foods])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
