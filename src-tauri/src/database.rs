use uuid::Uuid;

use crate::address::{Address, Gender};

pub struct Database {
    addresses: Vec<Address>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            addresses: vec![
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
            ],
        }
    }

    pub fn get_addresses(&self) -> Vec<Address> {
        self.addresses.clone()
    }

    pub fn upsert_address(&mut self, address: Address) {
        if let Some(found_address) = self.addresses.iter_mut().find(|d| d.id == address.id) {
            *found_address = address;
        } else {
            self.addresses.push(address);
        }
    }

    pub fn delete_address(&mut self, address: Address) {
        self.addresses.retain(|d| d.id != address.id);
    }
}
