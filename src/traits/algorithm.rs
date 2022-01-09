use std::rc::Rc;
use super::node::Node;

pub trait Algorithm<T: Node> {
    fn find_solution(&mut self, root: Box<T>) -> Option<Box<T>>;
    fn get_visited(&self) -> Vec<&Rc<T::State>>;
}