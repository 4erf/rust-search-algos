use std::collections::HashMap;
use crate::structs::container::Container;

#[derive(Clone)]
pub struct Extra {
    pub containers_inside_ship: HashMap<Container, usize>,
    pub containers_to_load: HashMap<Container, usize>,
}

impl Extra {
    pub fn containers_to_unload(&self, ship_port: usize) -> HashMap<Container, usize> {
        self.containers_inside_ship.iter()
            .filter(|(c, _)| c.dest_port == ship_port)
            .map(|(c, qnt)| (c.clone(), qnt.clone()))
            .collect()
    }
}