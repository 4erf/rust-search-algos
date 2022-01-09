# Labyrinth optimal path finder
### Build
`cargo build`

### Usage
`./target/debug/labyrinth`

Modify labyrinths directly from the [lays](./lays) folder, to change selected labyrinth change the following line on [main.rs](./src/main.rs#L199):
```rust
let path = Path::new("lays/bigMaze.lay");
```