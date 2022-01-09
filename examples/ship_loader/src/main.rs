pub mod utils;
pub mod structs;

use std::path::Path;
use std::rc::Rc;
use std::time::{Instant};

use search::traits::algorithm::Algorithm;
use search::bfs::BFS;
use search::dfs::DFS;
use search::dijkstra::Dijkstra;
use search::a_star::AStar;
use search::traits::node::Node;

use utils::parse;
use utils::output;

use structs::node::ShipNode;
use structs::ship::Ship;

fn get_help(exe: &str) -> String {
    format!("Usage:\n{} <path> <map> <containers> <heuristic-name> [search-algorithm]\n", exe)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 && args.len() != 6 {
        println!("\nInvalid number of arguments\n\n{}", get_help(&args[0]));
        return;
    }

    // Load input files

    let containers_path = Path::new(&args[1]).join(args[3].as_str()).into_boxed_path();
    let ship_map_path = Path::new(&args[1]).join(args[2].as_str()).into_boxed_path();
    let heuristic_name = args[4].as_str();
    let search_algo_name = args.get(5).map(|s| s.as_str()).unwrap_or("");

    let containers = parse::parse_containers(&containers_path).unwrap();
    let ship_stacks = parse::parse_ship_map(&ship_map_path).unwrap();

    // Build problem

    let ship = Ship::new(containers, ship_stacks, heuristic_name.to_owned());

    let mut search_algo: Box<dyn Algorithm<ShipNode>> = match search_algo_name {
        "bfs" => Box::new(BFS::new()),
        "dfs" => Box::new(DFS::new()),
        "dijkstra" => Box::new(Dijkstra::new()),
        "a_star" | _ => Box::new(AStar::new()),
    };

    let root = ShipNode {
        state: Rc::new(ship.build_initial_state()),
        action: None,
        cost: 0,
        parent: None,
        extra: ship.build_initial_extra(),
        ship: &ship,
    };

    // Get solution

    let time_start = Instant::now();
    let solution = search_algo.find_solution(Box::new(root)).expect("No solution was found");
    let duration = time_start.elapsed();

    let visited = search_algo.get_visited();

    let mut last = Rc::new(*solution);
    let mut nodes: Vec<Rc<ShipNode>> = Vec::new();
    let cost = last.cost;

    loop {
        nodes.push(last.clone());
        match last.get_parent() {
            Some(parent) => last = parent,
            None => break
        }
    }

    // Write output files

    let actions_path = Path::new(&args[1])
        .join(format!("{}-{}-{}.output", args[2], args[3], args[4]))
        .into_boxed_path();
    let stats_path = Path::new(&args[1])
        .join(format!("{}-{}-{}.stat", args[2], args[3], args[4]))
        .into_boxed_path();

    let mut actions_str = String::new();
    let mut stats_str = String::new();

    for node in nodes.iter().rev() {
        if let Some(action) = node.action.map(|a| ship.format_action(&a)) {
            actions_str = format!("{}{}\n", actions_str, action);
        }
    }

    if duration.as_secs() > 1 {
        stats_str = format!("{}Duration: {:.2}s\n", stats_str, duration.as_millis() as f64 / 1000.0);
    } else {
        stats_str = format!("{}Duration: {:.2}ms\n", stats_str, duration.as_micros() as f64 / 1000.0);
    }
    stats_str = format!("{}Length: {}\n", stats_str, nodes.len());
    stats_str = format!("{}Visited: {}\n", stats_str, visited.len());
    stats_str = format!("{}Cost: {}\n", stats_str, ship.calc_final_cost(cost));
    stats_str = format!("{}Cost w/o navigation: {}\n", stats_str, cost);

    output::write_string_to_file(&actions_path, actions_str).expect("Failed writing actions file");
    output::write_string_to_file(&stats_path, stats_str).expect("Failed writing stats file");
}