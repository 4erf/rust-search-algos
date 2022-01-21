use super::traits::node::Node;
use super::traits::algorithm::Algorithm;
use std::collections::{VecDeque, HashSet};
use std::rc::Rc;

pub struct BFS<T: Node> {
    queue: VecDeque<Box<T>>,
    visited: HashSet<Rc<T::State>>,
}

impl<T: Node> BFS<T> {
    pub fn new() -> Self {
        BFS { queue: VecDeque::new(), visited: HashSet::new() }
    }
}

impl<T: Node> Algorithm<T> for BFS<T> {
    fn find_solution(&mut self, root: Box<T>) -> Option<Box<T>> {
        self.visited.insert(root.get_state());
        self.queue.push_front(root);
        while let Some(node) = self.queue.pop_back() {
            if node.is_solution() { return Some(node) }
            for descendant in node.get_descendants() {
                if self.visited.contains(&descendant.get_state()) { continue; }
                self.visited.insert(descendant.get_state().clone());
                self.queue.push_front(descendant);
            }
        }
        None
    }

    fn get_visited(&self) -> Vec<&Rc<T::State>> {
        self.visited.iter().collect()
    }
}