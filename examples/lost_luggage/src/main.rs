pub mod structs;
pub mod enums;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;

use structs::location::Location;
use structs::node::Node;
use structs::flight::Flight;
use structs::luggage::Luggage;

use enums::action::Action;

use search::traits::algorithm::Algorithm;
use search::traits::node::Node as SearchNode;
use search::bfs::BFS;
use search::dfs::DFS;
use search::dijkstra::Dijkstra;
use search::a_star::AStar;
use search::dfbnb::DFBnB;

fn process_luggage(
    luggage: &Luggage,
    locations_map: &HashMap<String, &Location>,
    flights_map: &HashMap<String, &Flight>
) {
    let root = Node {
        location: Rc::new(luggage.origin.clone()),
        action: None,
        parent: None,
        time: 0,
        luggage: &luggage,
        locations: &locations_map,
    };
    let mut algo: Dijkstra<Node> = Dijkstra::new();
    let solution = algo.find_solution(Box::new(root)).expect("No solution found");

    let mut nodes: Vec<Rc<Node>> = Vec::new();
    let mut last = Rc::new(*solution);

    loop {
        nodes.push(last.clone());
        match last.get_parent() {
            Some(parent) => last = parent,
            None => break
        }
    }

    println!("Solution for luggage {}:", luggage.id);
    println!("Time: {}", nodes[0].time);
    println!("Visited: {}", algo.get_visited().len());
    println!("Plan:");
    for node in nodes.iter().rev() {
        if let Some(Action::Flight(flight)) = node.action.as_ref() {
            println!(
                "\tFlight {}->{} at time: {}. With: {}. Duration: {}",
                flights_map.get(flight).unwrap().origin,
                flights_map.get(flight).unwrap().destination,
                flights_map.get(flight).unwrap().departure,
                flight.split('-').collect::<Vec<&str>>()[0],
                flights_map.get(flight).unwrap().duration,
            );
        }
    }
    println!();
}

fn main() {
    let path = Path::new("configs/config.json");
    let file = File::open(path).expect("Can't open config file");
    let reader = BufReader::new(file);
    let locations: Vec<Location> = serde_json::from_reader(reader)
        .expect("Can't parse locations");
    let locations_map: HashMap<String, &Location> = locations
        .iter()
        .map(|location| (location.id.clone(), location))
        .collect();
    let flights_map: HashMap<String, &Flight> = locations
        .iter()
        .map(|location|
            location.flights.iter().map(|flight| (flight.id.clone(), flight))
        )
        .flatten()
        .collect();

    // Don't use with depth-first algos yet, huge RAM usage because of recursion.
    // TODO: Improve recursion on depth-first algos, maybe tail recursion if possible.
    rayon::scope(|s| {
        for location in locations.iter() {
            for luggage in location.luggage.iter() {
                s.spawn(|_| process_luggage(luggage, &locations_map, &flights_map));
            }
        }
    })
}