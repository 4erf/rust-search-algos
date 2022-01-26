use super::traits::node::Node;
use super::traits::algorithm::Algorithm;
use std::rc::Rc;
use std::collections::{HashMap};

/*
 *  You can use this algorithm ONLY if the cost function is monotonic
 */

pub struct DFBnB<T: Node> {
    visited: HashMap<Rc<T::State>, isize>,
}

impl<T: Node> DFBnB<T> {
    pub fn new() -> Self {
        DFBnB { visited: HashMap::new() }
    }

    fn find_solution_recurse(&mut self, node: Box<T>, mut best: Option<Box<T>>) -> Option<Box<T>> {
        for descendant in node.get_descendants() {
            if descendant.is_solution() {
                if let Some(best) = &best {
                    if descendant.get_cost() >= best.get_cost() { continue; }
                }
                best = Some(descendant);
                continue
            }

            if let Some(best) = &best {
                if descendant.get_cost() >= best.get_cost() { continue; }
            }

            if let Some(visited_cost) = self.visited.get(&descendant.get_state()) {
                if visited_cost < &descendant.get_cost() { continue; }
            }

            self.visited.insert(descendant.get_state(), descendant.get_cost());
            best = self.find_solution_recurse(descendant, best);
        }
        best
    }
}

impl<T: Node> Algorithm<T> for DFBnB<T> {
    fn find_solution(&mut self, root: Box<T>) -> Option<Box<T>> {
        if root.is_solution() { return Some(root); }
        self.visited.insert(root.get_state(), root.get_cost());
        self.find_solution_recurse(root, None)
    }

    fn get_visited(&self) -> Vec<&Rc<T::State>> {
        self.visited.keys().collect()
    }
}