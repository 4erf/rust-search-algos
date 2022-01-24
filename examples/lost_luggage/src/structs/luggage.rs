use serde::{Deserialize};
use super::flight::Flight;

#[derive(Deserialize)]
pub struct Luggage {
    pub id: String,
    pub origin: String,
    pub destination: String,
}