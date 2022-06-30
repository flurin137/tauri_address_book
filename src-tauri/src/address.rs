use std::fmt::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub address: String,
    pub gender: Gender,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} | {} | {}", self.id, self.name, self.address)
    }
}
