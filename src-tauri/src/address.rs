use serde::{Deserialize, Serialize};
use std::fmt::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub address: String,
    pub gender: Gender,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} | {} | {} | {:?}",
            self.id, self.name, self.address, self.gender
        )
    }
}
