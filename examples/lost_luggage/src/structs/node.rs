use std::rc::Rc;
use std::collections::{HashMap};
use super::location::Location;
use crate::enums::action::Action;
use crate::structs::luggage::Luggage;

pub struct Node<'a> {
    pub location: Rc<String>,
    pub action: Option<Action>,
    pub parent: Option<Rc<Node<'a>>>,
    pub time: usize,
    /* Shared */
    pub luggage: &'a Luggage,
    pub locations: &'a HashMap<String, &'a Location>,
}

impl<'a> search::traits::node::Node for Node<'a> {
    type State = String;
    type Action = Action;

    fn get_descendants(self) -> Vec<Box<Self>> {
        let this = Rc::new(self);
        this.locations
            .get(this.location.as_ref())
            .unwrap()
            .flights
            .iter()
            .filter(|flight| flight.departure >= this.time)
            .map(|flight| {
                Box::new(Node {
                    location: Rc::new(flight.destination.clone()),
                    action: Some(Action::Flight(flight.id.clone())),
                    parent: Some(this.clone()),
                    time: flight.departure + flight.duration,
                    locations: this.locations,
                    luggage: this.luggage,
                })
            })
            .collect()
    }

    fn get_state(&self) -> Rc<Self::State> {
        self.location.clone()
    }

    fn get_action(&self) -> Option<Self::Action> {
        self.action.clone()
    }

    fn get_parent(&self) -> Option<Rc<Self>> {
        self.parent.clone()
    }

    fn get_cost(&self) -> isize {
        self.time.try_into().expect("Cost overflowed")
    }

    fn get_heuristic(&self) -> isize {
        0
    }

    fn is_solution(&self) -> bool {
        *self.location == self.luggage.destination
    }
}