use super::flight::Flight;
use super::luggage::Luggage;

pub struct Location {
    id: String,
    luggage: Vec<Luggage>,
    flights: Vec<Flight>,
}