use std::rc::Rc;
use search::traits::node::Node;
use super::extra::Extra;
use super::ship::Ship;
use super::state::State;
use super::action::Action;

pub struct ShipNode<'a> {
    pub state: Rc<State>,
    pub action: Option<Action>,
    pub cost: isize,
    pub parent: Option<Rc<Self>>,
    pub extra: Extra,
    pub ship: &'a Ship,
}

impl<'a> Node for ShipNode<'a> {
    type State = State;
    type Action = Action;

    fn get_descendants(self) -> Vec<Box<Self>> {
        let mut descendants: Vec<Box<ShipNode>> = Vec::new();
        let parent = Rc::new(self);
        for action in parent.ship.get_possible_actions(&parent.state, &parent.extra) {
            let (state, extra) = parent.ship.take_action(&parent.state, &parent.extra, &action);
            descendants.push(Box::new(ShipNode {
                state: Rc::new(state),
                action: Some(action),
                cost: parent.cost + action.get_cost(),
                parent: Some(parent.clone()),
                extra,
                ship: parent.ship,
            }))
        }
        descendants
    }

    fn get_state(&self) -> Rc<State> { self.state.clone() }
    fn get_action(&self) -> Option<Action> { self.action }
    fn get_parent(&self) -> Option<Rc<Self>> { self.parent.clone() }
    fn get_cost(&self) -> isize { self.cost }
    fn get_heuristic(&self) -> isize {
        self.ship.get_heuristic_value(&self.state, &self.extra)
    }
    fn is_solution(&self) -> bool {
        // if let Some(action) = &self.action {
        //     println!("{}", self.ship.format_action(action));
        // }
        // println!("{}", self.ship.format_state(&self.state));
        self.ship.is_state_solution(&self.state)
    }
}

