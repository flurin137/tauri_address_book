#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_addresses])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_addresses() -> Vec<Address> {
    vec![
        Address {
            name: "12341".to_owned(),
            address: "".to_owned(),
            age: 12,
            gender: Gender::Female,
        },
        Address {
            name: "rqew".to_owned(),
            address: "".to_owned(),
            age: 12,
            gender: Gender::Female,
        },
        Address {
            name: "zurrzt".to_owned(),
            address: "".to_owned(),
            age: 12,
            gender: Gender::Female,
        },
        Address {
            name: "asdf".to_owned(),
            address: "".to_owned(),
            age: 12,
            gender: Gender::Female,
        },
    ]
}

#[derive(Serialize)]
pub struct Address {
    name: String,
    address: String,
    gender: Gender,
    age: i32,
}

#[derive(Serialize)]
pub enum Gender {
    Male,
    Female,
}
