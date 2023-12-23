use serde::{Deserialize, Serialize};
pub use super::db_types::Role;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,
    pub login: String,
    pub registered_at: String,
    pub role_id: u8,
}