use super::flight::Flight;

pub struct Luggage {
    id: String,
    origin: String,
    destination: String,
    plan: Vec<Flight>,
}