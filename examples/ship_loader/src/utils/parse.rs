use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::path::Path;
use crate::structs::cell::{Cell, CellType};
use crate::structs::container::{Container, ContainerType};

pub fn parse_containers(path: &Path) -> Result<Vec<Container>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut containers: Vec<Container> = Vec::new();
    for line_res in lines {
        let line = line_res?;
        if line.is_empty() { continue; }
        let chars: Vec<String> = line.split('\x20').map(|str| str.to_owned()).collect();
        let char = chars.get(1)
            .ok_or(std::io::Error::new(ErrorKind::InvalidInput, "Can't get container char"))?
            .parse::<char>()?;
        let port = chars.get(2)
            .ok_or(std::io::Error::new(ErrorKind::InvalidInput, "Can't get container port"))?
            .parse::<usize>()?;
        let container_type = ContainerType::try_from(char)?;

        containers.push(Container { type_: container_type, dest_port: port });
    }
    Ok(containers)
}

pub fn parse_ship_map(path: &Path) -> Result<Vec<Vec<Cell>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut cells: Vec<Vec<Cell>> = Vec::new();

    let mut row: Vec<Cell> = Vec::new();
    let buffer = &mut[0 as u8];
    while let Ok(()) = reader.read_exact(buffer) {
        match buffer[0] as char {
            '\x20' => (),
            '\n' => {
                if row.is_empty() { continue; }
                cells.push(row);
                row = Vec::new();
            }
            c => {
                row.push(Cell { type_: CellType::try_from(c)? })
            }
        }
    }
    if !row.is_empty() {
        cells.push(row);
    }

    Ok(depths_to_stacks(cells))
}

pub fn depths_to_stacks<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    /*
     *  This goes from depths (initial format):
     *  N N E N
     *  X E X N
     *  X X X X
     *
     *  To stacks (reversed, bottom-up order):
     *  X X N
     *  X E N
     *  X X E
     *  X N N
     */
    if v.is_empty() { return v; }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().rev().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().expect("Vec<Vec<T>> is not a matrix"))
                .collect::<Vec<T>>()
        })
        .collect()
}