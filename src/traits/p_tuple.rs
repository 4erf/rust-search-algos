use std::cmp::{Ordering};

pub struct PTuple<T> {
    pub priority: isize,
    pub node: T,
}

impl<T> Ord for PTuple<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Inverse order: min goes first
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialOrd for PTuple<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Inverse order: min goes first
        other.priority.partial_cmp(&self.priority)
    }
}

impl<T> PartialEq for PTuple<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for PTuple<T> { }