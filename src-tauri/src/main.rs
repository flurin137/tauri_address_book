#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod address;
mod database;

use std::{borrow::BorrowMut, sync::Mutex};

use address::Address;
use database::Database;
use tauri::State;

fn main() {
    let context = tauri::generate_context!();
    let database = Mutex::new(Database::new());

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![get_addresses, save_address])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_addresses(database: State<Mutex<Database>>) -> Vec<Address> {
    match database.lock() {
        Ok(database) => return database.get_addresses(),
        Err(_) => return vec![],
    }
}

#[tauri::command]
fn save_address(database: State<Mutex<Database>>, address: Address) {
    match database.lock() {
        Ok(mut database) => return database.set_address(address),
        Err(_) => (),
    }
}
