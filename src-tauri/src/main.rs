#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod address;
mod database;

use std::{sync::Mutex, time};

use address::Address;
use database::Database;
use serde::Serialize;
use tauri::{State, Window};

#[derive(Serialize, Clone)]
struct EventPayload {
    message: String,
}

fn main() {
    let context = tauri::generate_context!();
    let database = Mutex::new(Database::new());

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            init_process,
            get_addresses,
            save_address,
            delete_address
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_process(window: Window) {
    std::thread::spawn(move || loop {
        window
            .emit(
                "Yeeehaaa",
                EventPayload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .ok();

        std::thread::sleep(time::Duration::from_secs(1));
    });
}

#[tauri::command]
fn get_addresses(database: State<Mutex<Database>>) -> Vec<Address> {
    match database.lock() {
        Ok(database) => database.get_addresses(),
        Err(_) => vec![],
    }
}

#[tauri::command]
fn save_address(database: State<Mutex<Database>>, address: Address) {
    println!("{}", &address);
    if let Ok(mut database) = database.lock() {
        database.upsert_address(address)
    }
}

#[tauri::command]
fn delete_address(database: State<Mutex<Database>>, address: Address) {
    if let Ok(mut database) = database.lock() {
        database.delete_address(address)
    }
}
