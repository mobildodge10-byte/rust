use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Medicine {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub medicine_type_id: i32,
    pub dosage_form: String,
}