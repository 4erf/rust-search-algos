# Rust search algorithms
Couple of search algorithms implemented in the rust language.

### Algorithms included
- BFS
- DFS
- Dijkstra
- A*

You can fork this repo and implement your own using the provided data structures and interfaces.

### Usage
Add this to your Cargo.toml `[dependencies]` section:
```toml
regex = { git = "https://github.com/4erf/rust-search-algos" }
```

In your app implement the `Node` trait and create root node, ex:
```rust
impl search::Node for CustomNode { ... }
...
let root = CustomNode { ... };
```

Finally instantiate the required search algorithm and find a solution:
```rust
let algo = search::AStar::new();
let solution = algo.find_solution(Box::new(root)).expect("No solution was found");
```

The solution will be a reference to the last (solution) node, to get the whole path traverse backwards:
```rust
let mut last = Rc::new(*solution);
let cost = last.cost;

let mut nodes: Vec<Rc<CustomNode>> = Vec::new();
loop {
    nodes.push(last.clone());
    match last.get_parent() {
        Some(parent) => last = parent,
        None => break
    }
}
```

You can also get a list of all visited nodes:
```rust
let visited = search_algo.get_visited();
```

### Dependencies
None

### License
MIT