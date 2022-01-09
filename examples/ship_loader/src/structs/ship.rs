use std::collections::HashMap;

use super::action::{Action, ActionType};
use super::cell::{Cell, CellType};
use super::container::{Container, ContainerType};
use super::state::State;
use super::extra::Extra;

pub struct Ship {
    container_counts: HashMap<Container, usize>,
    stacks: Vec<Vec<Cell>>,
    heuristic_name: String,
    last_port: usize,
    stack_height: usize,
}

impl Ship {
    pub fn new(
        containers: Vec<Container>,
        stacks: Vec<Vec<Cell>>,
        heuristic_name: String,
    ) -> Self {
        let last_port = containers.iter().map(|c| c.dest_port).max().unwrap_or(0);
        let mut container_counts: HashMap<Container, usize> = HashMap::new();
        for container in containers {
            container_counts.insert(container, 1 + container_counts.get(&container).unwrap_or(&0));
        }
        let stack_height = stacks.get(0).map(|s| s.len()).unwrap_or(0);
        Ship { container_counts, stacks, heuristic_name, last_port, stack_height }
    }

    pub fn build_initial_state(&self) -> State {
        State {
            container_pos: self.stacks.iter()
                .map(|s| s.iter().map(|_| None).collect())
                .collect(),
            ship_port: 0,
        }
    }

    pub fn build_initial_extra(&self) -> Extra {
        let state = self.build_initial_state();
        Extra {
            containers_inside_ship: self.get_containers_inside_ship(&state),
            containers_to_load: self.get_containers_to_load(&state),
        }
    }

    pub fn format_state(&self, state: &State) -> String {
        let mut formatted = String::new();
        for (i, stack) in self.stacks.iter().enumerate() {
            for (j, cell) in stack.iter().enumerate() {
                let cell_char: char = cell.type_.into();
                let char: String = state.container_pos[i][j]
                    .map(|c| {
                        let cont_char: char = c.type_.into();
                        format!("({},{})", cont_char, c.dest_port.to_string())
                    })
                    .unwrap_or(String::from(cell_char));
                formatted = format!("{}{:<8}", formatted, char);
            }
            formatted += "\n";
        }
        formatted = format!("{}Port: {}", formatted, state.ship_port);
        formatted
    }

    pub fn format_action(&self, action: &Action) -> String {
        format!(
            "{} (Container type: {}, Destination port: {}, Stack: {}, Movement: {})",
            match action.type_ {
                ActionType::Load => "Load",
                ActionType::Unload => "Unload",
                ActionType::Navigate => "Navigate",
            },
            action.container.map(|c| match c.type_ {
                ContainerType::Standard => "Standard",
                ContainerType::Refrigerated => "Refrigerated",
            }).unwrap_or("<NA>"),
            action.container.map(|c| c.dest_port.to_string()).unwrap_or("<NA>".to_owned()),
            action.stack.map(|s| s.to_string()).unwrap_or("<NA>".to_owned()),
            action.movement.map(|m| m.to_string()).unwrap_or("<NA>".to_owned()),
        )
    }

    pub fn is_state_solution(&self, state: &State) -> bool {
        if state.ship_port != self.last_port { return false; }
        state.container_pos.iter().all(|row| row.iter().all(|c| c.is_none()))
    }

    fn get_containers_inside_ship(&self, state: &State) -> HashMap<Container, usize> {
        let mut inside_ship: HashMap<Container, usize> = HashMap::new();
        for stack in &state.container_pos {
            for container in stack.iter().flatten() {
                inside_ship.insert(*container, 1 + inside_ship.get(container).unwrap_or(&0));
            }
        }
        inside_ship
    }

    fn get_containers_to_load(&self, state: &State) -> HashMap<Container, usize> {
        let loaded_count = self.get_containers_inside_ship(state);
        let mut to_load: HashMap<Container, usize> = HashMap::new();
        for dest_port in state.ship_port + 1..self.last_port + 1 {
            for type_ in ContainerType::get_all() {
                let container = Container { type_, dest_port };
                if let Some(qnt) = self.container_counts.get(&container) {
                    let loaded = loaded_count.get(&container).unwrap_or(&0);
                    to_load.insert(container, qnt - loaded);
                }
            }
        }
        to_load
    }

    fn get_usable_cells(&self, state: &State) -> Vec<(Cell, usize, usize)> {
        let mut cells: Vec<(Cell, usize, usize)> = Vec::new();
        'outer: for (s, stack) in self.stacks.iter().enumerate() {
            for (d, cell) in stack.iter().enumerate() {
                let available = cell.type_ != CellType::Unavailable;
                let occupied = state.container_pos[s][d].is_some();
                if !available || occupied { continue; }
                cells.push((*cell, s, d));
                continue 'outer;
            }
        }
        cells
    }

    fn get_unloadable_containers(&self, state: &State) -> Vec<(Container, usize, usize)> {
        let mut containers: Vec<(Container, usize, usize)> = Vec::new();
        'outer: for (s, stack) in self.stacks.iter().enumerate() {
            for (d, cell) in stack.iter().enumerate().rev() {
                if cell.type_ == CellType::Unavailable { continue; }
                if let Some(container) = &state.container_pos[s][d] {
                    containers.push((*container, s, d));
                    continue 'outer;
                }
            }
        }
        containers
    }

    pub fn get_possible_actions(&self, state: &State, extra: &Extra) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        let are_containers_to_load = extra.containers_to_load.values().any(|qnt| *qnt != 0);
        let are_containers_to_unload = extra.containers_to_unload(state.ship_port)
            .values().any(|qnt| *qnt != 0);

        // Action 1: If no containers are left to load or unload
        if !are_containers_to_load && !are_containers_to_unload {
            return vec![Action {
                type_: ActionType::Navigate,
                container: None,
                stack: None,
                movement: None,
            }]
        }

        // println!(
        //     "To load: {}\nTo unload: {}\n",
        //     extra.containers_to_load.values().map(|v| *v).reduce(|acc, qnt| acc + qnt).unwrap_or(0),
        //     extra.containers_to_unload(state.ship_port).values().map(|v| *v).reduce(|acc, qnt| acc + qnt).unwrap_or(0),
        // );

        // Action 2: Containers that have to be loaded
        if are_containers_to_load {
            let usable_cells = self.get_usable_cells(&state);
            for (cell, s, d) in usable_cells {
                let compatible_container_types = cell.type_.get_compatible_containers();
                for (Container { type_, dest_port }, qnt) in extra.containers_to_load.iter() {
                    if *qnt == 0 { continue; }
                    if !compatible_container_types.contains(&type_) { continue; }
                    actions.push(Action {
                        type_: ActionType::Load,
                        container: Some(Container { type_: *type_, dest_port: *dest_port }),
                        stack: Some(s),
                        movement: Some(self.stack_height - d),
                    });
                }
            }
        }

        // Action 3: Containers that can be unloaded
        if are_containers_to_unload {
            let unloadable_containers = self.get_unloadable_containers(&state);
            for (container, s, d) in unloadable_containers {
                actions.push(Action {
                    type_: ActionType::Unload,
                    container: Some(container),
                    stack: Some(s),
                    movement: Some(self.stack_height - d),
                });
            }
        }

        actions
    }

    pub fn take_action(&self, state: &State, extra: &Extra, action: &Action) -> (State, Extra) {
        let mut new = state.clone();
        let mut new_extra = extra.clone();
        match action.type_ {
            ActionType::Navigate => new.ship_port += 1,
            ActionType::Load => {
                let stack = action.stack.unwrap();
                let depth = self.stack_height - action.movement.unwrap();
                new.container_pos[stack][depth] = action.container;
                let container = action.container.unwrap();
                *new_extra.containers_inside_ship.entry(container).or_insert(0) += 1;
                *new_extra.containers_to_load.entry(container).or_insert(0) -= 1;
            }
            ActionType::Unload => {
                let stack = action.stack.unwrap();
                let depth = self.stack_height - action.movement.unwrap();
                new.container_pos[stack][depth] = None;
                let container = action.container.unwrap();
                *new_extra.containers_inside_ship.entry(container).or_insert(0) -= 1;
                if container.dest_port != state.ship_port {
                    *new_extra.containers_to_load.entry(container).or_insert(0) += 1;
                }
            }
        }
        (new, new_extra)
    }

    pub fn get_load_unload_heuristic(&self, extra: &Extra) -> isize {
        let containers_to_load: isize = extra.containers_to_load.values()
            .map(|v| *v).reduce(|acc, qnt| acc + qnt)
            .unwrap_or(0).try_into().expect("Cost overflowed");
        let containers_inside_ship: isize = extra.containers_inside_ship.values()
            .map(|v| *v).reduce(|acc, qnt| acc + qnt)
            .unwrap_or(0).try_into().expect("Cost overflowed");

        let ctl_factor = Action {
            type_: ActionType::Load,
            container: None,
            stack: None,
            movement: Some(1),
        }.get_cost();

        let cis_factor = Action {
            type_: ActionType::Unload,
            container: None,
            stack: None,
            movement: Some(1),
        }.get_cost();

        containers_to_load * ctl_factor
        + containers_to_load * cis_factor
        + containers_inside_ship * cis_factor
    }

    pub fn get_fast_heuristic(&self, state: &State) -> isize {
        let cis_factor = Action {
            type_: ActionType::Unload,
            container: None,
            stack: None,
            movement: Some(1),
        }.get_cost();

        let mut count = 0;
        for dest_port in state.ship_port + 1..self.last_port + 1 {
            for type_ in ContainerType::get_all() {
                let container = Container { type_, dest_port };
                if let Some(qnt) = self.container_counts.get(&container) {
                    count += qnt;
                }
            }
        }
        count.try_into().map(|c: isize| c * cis_factor).expect("Heuristic count overflowed")
    }

    pub fn get_null_heuristic(&self) -> isize {
        0
    }

    pub fn get_heuristic_value(&self, state: &State, extra: &Extra) -> isize {
        match self.heuristic_name.as_str() {
            "heuristica1" => self.get_load_unload_heuristic(extra),
            "heuristica2" => self.get_fast_heuristic(state),
            "null" | _ => self.get_null_heuristic(),
        }
    }

    pub fn calc_final_cost(&self, cost: isize) -> isize {
        cost + isize::try_from(self.last_port * 3500).expect("Cost overflowed")
    }
}