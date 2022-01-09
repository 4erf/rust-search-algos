use std::hash::Hash;
use std::rc::Rc;

pub trait Node {
    type State: Eq + Hash;
    type Action;

    fn get_descendants(self) -> Vec<Box<Self>>;
    fn get_state(&self) -> Rc<Self::State>;
    fn get_action(&self) -> Option<Self::Action>;
    fn get_parent(&self) -> Option<Rc<Self>>;
    fn get_cost(&self) -> isize;
    fn get_heuristic(&self) -> isize;
    fn is_solution(&self) -> bool;
}