# Lost luggage recovery planner

It takes a series of airports with their next flights and a list of lost luggage in each of them, 
then gives the series of flights for each luggage to arrive its destination as soon as possible.

An example initial state can be found on [test.json](./configs/test.json).

### Build
`cargo build --release`

### Dependencies
[rayon](https://crates.io/crates/rayon) for multithreading

[serde](https://crates.io/crates/serde) and 
[serde_json](https://crates.io/crates/serde_json) 
for json serialization
