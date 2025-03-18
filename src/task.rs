use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
