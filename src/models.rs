use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactEntry {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub message: String,
}
