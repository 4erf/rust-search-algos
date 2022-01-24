use serde::{Deserialize};
use super::flight::Flight;
use super::luggage::Luggage;

#[derive(Deserialize)]
pub struct Location {
    pub id: String,
    pub flights: Vec<Flight>,
    pub luggage: Vec<Luggage>,
}