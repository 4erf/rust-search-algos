use super::traits::node::Node;
use super::traits::algorithm::Algorithm;
use std::collections::{HashSet};
use std::rc::Rc;

pub struct DFS<T: Node> {
    visited: HashSet<Rc<T::State>>,
}

impl<T: Node> DFS<T> {
    pub fn new() -> Self {
        DFS { visited: HashSet::new() }
    }

    fn find_solution_recurse(&mut self, node: Box<T>) -> Option<Box<T>> {
        if node.is_solution() { return Some(node) }
        for descendant in node.get_descendants() {
            if self.visited.contains(&descendant.get_state()) { continue; }
            self.visited.insert(descendant.get_state().clone());
            if let Some(solution) = self.find_solution_recurse(descendant) {
                return Some(solution)
            }
        }
        None
    }
}

impl<T: Node> Algorithm<T> for DFS<T> {
    fn find_solution(&mut self, root: Box<T>) -> Option<Box<T>> {
        self.visited.insert(root.get_state());
        self.find_solution_recurse(root)
    }

    fn get_visited(&self) -> Vec<&Rc<T::State>> {
        self.visited.iter().collect()
    }
}