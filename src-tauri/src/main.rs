#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fmt::Display;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_addresses, save_address])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_addresses() -> Vec<Address> {
    vec![
        Address {
            id: Uuid::new_v4(),
            name: "12341".to_owned(),
            email: "hans@nötig.com".to_owned(),
            address: "wqeh rhjqwgrk qwerjhg qwek".to_owned(),
            gender: Gender::Female,
        },
        Address {
            id: Uuid::new_v4(),
            name: "rqew".to_owned(),
            email: "hans@nötig.com".to_owned(),
            address: "b ewkjhrgjhqweg rkjhqweg ".to_owned(),
            gender: Gender::Female,
        },
    ]
}

#[tauri::command]
fn save_address(address: Address) {
    println!("{address}");
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    id: Uuid,
    name: String,
    email: String,
    address: String,
    gender: Gender,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {} | {}", self.id, self.name, self.address)
    }
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}
