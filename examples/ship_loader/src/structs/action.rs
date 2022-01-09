use super::container::Container;

#[derive(Copy, Clone)]
pub enum ActionType { Load, Unload, Navigate }

#[derive(Copy, Clone)]
pub struct Action {
    pub type_: ActionType,
    pub container: Option<Container>,
    pub stack: Option<usize>,
    pub movement: Option<usize>, // Is complement of depth
}

impl Action {
    pub fn get_cost(&self) -> isize {
        isize::try_from(match self.type_ {
            ActionType::Load => 10 + self.movement.unwrap(),
            ActionType::Unload => 15 + 2 * self.movement.unwrap(),
            ActionType::Navigate => 0,
        }).expect("Cost has overflowed")
    }
}