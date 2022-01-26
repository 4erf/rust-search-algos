use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Luggage {
    pub id: String,
    pub origin: String,
    pub destination: String,
}