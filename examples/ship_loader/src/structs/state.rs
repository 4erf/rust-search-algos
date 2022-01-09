use super::container::Container;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct State {
    pub container_pos: Vec<Vec<Option<Container>>>,
    pub ship_port: usize,
}