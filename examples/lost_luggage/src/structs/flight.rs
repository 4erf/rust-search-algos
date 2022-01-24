use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct Flight {
    pub id: String,
    pub origin: String,
    pub destination: String,
    pub departure: usize,
    pub duration: usize,
}