use super::traits::{
    node::Node,
    algorithm::Algorithm,
    p_tuple::PTuple
};
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

pub struct AStar<T: Node> {
    p_queue: BinaryHeap<PTuple<Box<T>>>,
    visited: HashMap<Rc<T::State>, isize>,
}

impl<T: Node> AStar<T> {
    pub fn new() -> Self {
        AStar {
            p_queue: BinaryHeap::new(),
            visited: HashMap::new(),
        }
    }

    fn cost_fn(n: &Box<T>) -> isize { n.get_cost() + n.get_heuristic() }
}

impl<T: Node> Algorithm<T> for AStar<T> {
    fn find_solution(&mut self, root: Box<T>) -> Option<Box<T>> {
        self.visited.insert(root.get_state(), root.get_cost());
        self.p_queue.push(PTuple { priority: AStar::cost_fn(&root), node: root });

        while let Some(PTuple {priority: _, node}) = self.p_queue.pop() {
            if node.is_solution() { return Some(node) }
            for descendant in node.get_descendants() {
                let visited_cost = *self.visited.get(&descendant.get_state()).unwrap_or(&isize::MAX);
                if visited_cost <= AStar::cost_fn(&descendant) { continue; }
                self.visited.insert(descendant.get_state().clone(), descendant.get_cost());
                self.p_queue.push(PTuple { priority: AStar::cost_fn(&descendant), node: descendant });
            }
        }
        None
    }

    fn get_visited(&self) -> Vec<&Rc<T::State>> {
        self.visited.keys().collect()
    }
}