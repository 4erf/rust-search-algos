use std::fmt::{Display, Formatter};
use std::path::Path;
use std::io::{BufReader, Read};
use std::fs::File;
use std::rc::Rc;

use search::traits::node::Node;
use search::traits::algorithm::Algorithm;
use search::bfs::BFS;
use search::dfs::DFS;
use search::dijkstra::Dijkstra;
use search::a_star::AStar;

#[derive(Copy, Clone)]
enum Action { North, South, West, East }
#[derive(PartialEq)]
enum Cell { Wall, Empty, Pacman, Goal, Path, Visited }

impl Cell {
    fn from_char(char: char) -> Self {
        match char {
            '\x20' | '.' => Cell::Empty,
            '%' => Cell::Wall,
            'P' => Cell::Pacman,
            'G' => Cell::Goal,
            _ => panic!("Not a valid cell"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "\x20"),
            Cell::Wall => write!(f, "\u{2588}"),
            Cell::Pacman => write!(f, "P"),
            Cell::Goal => write!(f, "G"),
            Cell::Path => write!(f, "\x1b[1;31m\u{2588}\x1b[0m"),
            Cell::Visited => write!(f, "\x1b[1;34m\u{2588}\x1b[0m"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

struct Labyrinth {
    cells: Vec<Vec<Cell>>,
    goal: Option<Coord>,
}

impl Labyrinth {
    fn new(path: &Path) -> std::io::Result<Self> {
        let mut labyrinth = Labyrinth{ cells: Vec::new(), goal: None };
        match File::open(path) {
            Ok(file) => {
                let mut row: Vec<Cell> = Vec::new();
                let mut reader = BufReader::new(file);
                let buffer = &mut[0_u8];
                while let Ok(()) = reader.read_exact(buffer) {
                    match buffer[0] as char {
                        '\n' => {
                            if !row.is_empty() {
                                labyrinth.cells.push(row);
                                row = Vec::new();
                            }
                        }
                        c => {
                            row.push(Cell::from_char(c))
                        }
                    }
                }
                if !row.is_empty() {
                    labyrinth.cells.push(row);
                }
                labyrinth.goal = labyrinth.get_first_occurrence(&Cell::Goal).ok();
                Ok(labyrinth)
            }
            Err(e) => Err(e)
        }
    }

    fn format(&self) -> String {
        let mut formatted = String::new();
        for row in &self.cells {
            for cell in row {
                formatted = format!("{}{}", formatted, cell);
            }
            formatted += "\n";
        }
        formatted
    }

    fn get_first_occurrence(&self, target: &Cell) -> Result<Coord, String> {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell != target { continue; }
                return Ok(Coord { x: j, y: i })
            }
        }
        Err(format!("Element {} not found in labyrinth", target))
    }

    fn is_coord_stepable(&self, coord: &Coord) -> bool {
        if let Some(cell) = self.cells.get(coord.y).and_then(|row| row.get(coord.x)) {
            return cell != &Cell::Wall
        }
        false
    }

    fn take_action(&self, coord: &Coord, action: &Action) -> Option<Coord> {
        let mut result = coord.clone();
        match action {
            Action::North => result.y -= 1,
            Action::South => result.y += 1,
            Action::West => result.x -= 1,
            Action::East => result.x += 1,
        }
        if self.is_coord_stepable(&result) {
            return Some(result)
        }
        None
    }

    fn is_coord_goal(&self, coord: &Coord) -> bool {
        self.goal.as_ref().map(|g| g == coord).unwrap_or(false)
    }

    fn set_coord(&mut self, coord: &Coord, new: Cell) -> Result<(), ()> {
        if let Some(cell) = self.cells.get_mut(coord.y).and_then(|row| row.get_mut(coord.x)) {
            *cell = new;
            return Ok(());
        }
        Err(())
    }

    fn calc_manhattan_distance(&self, start: &Coord, end: &Coord) -> isize {
        let sx: isize = start.x.try_into().unwrap();
        let ex: isize = end.x.try_into().unwrap();
        let sy: isize = start.y.try_into().unwrap();
        let ey: isize = end.y.try_into().unwrap();
        isize::abs(sx - ex) + isize::abs(sy - ey)
    }

    fn calc_manhattan_distance_to_goal(&self, start: &Coord) -> isize {
        self.goal.as_ref().map(|end| self.calc_manhattan_distance(start, end)).unwrap_or(0)
    }

}

struct PacmanNode<'a> {
    state: Rc<Coord>,
    action: Option<Action>,
    cost: isize,
    parent: Option<Rc<Self>>,
    labyrinth: &'a Labyrinth,
}

impl<'a> Node for PacmanNode<'a> {
    type State = Coord;
    type Action = Action;

    fn get_descendants(self) -> Vec<Box<Self>> {
        static ACTIONS: [Action; 4] = [Action::North, Action::South, Action::West, Action::East];
        let mut descendants: Vec<Box<PacmanNode>> = Vec::new();
        let parent = Rc::new(self);
        for action in ACTIONS {
            if let Some(state) = parent.labyrinth.take_action(&parent.state, &action) {
                descendants.push(Box::new(PacmanNode {
                    state: Rc::new(state),
                    action: Some(action),
                    cost: parent.cost + 1,
                    parent: Some(parent.clone()),
                    labyrinth: parent.labyrinth,
                }))
            }
        }
        descendants
    }

    fn get_state(&self) -> Rc<Coord> { self.state.clone() }
    fn get_action(&self) -> Option<Action> { self.action }
    fn get_parent(&self) -> Option<Rc<Self>> { self.parent.clone() }
    fn get_cost(&self) -> isize { self.cost }

    fn get_heuristic(&self) -> isize {
        self.labyrinth.calc_manhattan_distance_to_goal(&self.state)
    }

    fn is_solution(&self) -> bool {
        self.labyrinth.is_coord_goal(&self.state)
    }
}

fn main() {
    let path = Path::new("_lays/bigMaze.lay");
    let labyrinth = Labyrinth::new(path).expect("Cannot load labyrinth");
    let pacman = labyrinth.get_first_occurrence(&Cell::Pacman).expect("Cannot find pacman");

    let node = PacmanNode {
        state: Rc::new(pacman),
        action: None,
        cost: 0,
        parent: None,
        labyrinth: &labyrinth
    };

    // let mut algo: BFS<PacmanNode> = BFS::new();
    // let solution = algo.find_solution(Box::new(node)).expect("No solution found");

    // let mut algo: DFS<PacmanNode> = DFS::new();
    // let solution = algo.find_solution(Box::new(node)).expect("No solution found");

    // let mut algo: Dijkstra<PacmanNode> = Dijkstra::new();
    // let solution = algo.find_solution(Box::new(node)).expect("No solution found");

    let mut algo: AStar<PacmanNode> = AStar::new();
    let solution = algo.find_solution(Box::new(node)).expect("No solution found");

    let mut result = Labyrinth::new(path).expect("Cannot load labyrinth");

    for visited in algo.get_visited() {
        result.set_coord(visited, Cell::Visited).expect("Can't set result coord");
    }

    let mut last = Rc::new(*solution);
    let mut length = 0;
    loop {
        result.set_coord(&last.get_state(), Cell::Path).expect("Can't set result coord");
        match last.get_parent() {
            Some(parent) => last = parent,
            None => break
        }
        length += 1;
    }

    println!("Initial:\n{}", labyrinth.format());
    println!("Final:\n{}", result.format());
    println!("Length: {}", length);
    println!("Visited: {}", algo.get_visited().len());
}
