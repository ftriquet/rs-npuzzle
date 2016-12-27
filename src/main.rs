extern crate clap;

mod node;
mod heuristics;

use node::Node;
use std::collections::BinaryHeap;
use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::Read;
use std::error::Error;

fn main() {
    let board = vec![
        8,  0,  9,  3,
        1,  6, 2, 4,
        12, 15, 5, 13,
        11, 14, 10, 7
    ];
    let n = node::Node {
        board: vec![1,  2,  3,
                    7,  8,  4,
                    6,  5,  0],
        //board: board,
        len: 3,
        heuristic: 0,
        cost: 0,
        parents: None,
    };

    let matches = App::new("Npuzzle")
        .about("Taquin solver")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("File containing the initial configuration")
             )
        .get_matches();

    let file_name = matches.value_of("file").unwrap_or("default.map");
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut s = String::new();
    file.read_to_string(&mut s);

    match s.parse::<node::Node>() {
        Ok(n) => solve(n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn solve(n: node::Node) {
    let goal: node::Node = node::Node::goal(n.len);

    let mut open: BinaryHeap<node::Node> = BinaryHeap::new();
    let mut closed: BinaryHeap<node::Node> = BinaryHeap::new();

    open.push(n);

    while let Some(node) = open.pop() {
        if node == goal {
            println!("Solved!");
            for p in node.clone().parents.unwrap() {
                let tmp = Node {
                    len: 3,
                    heuristic: 0,
                    cost: 0,
                    parents: None,
                    board: p.clone(),
                };
                tmp.print_grid();
            }
            node.print_grid();
            break
        } else {
            let neighbours = node.get_next_steps();

            for neighbour in neighbours {
                match open.iter().find(|&node| *node == neighbour)
                    .or(closed.iter().find(|&n| *n == neighbour)) {
                        Some(_) => {},
                        None => {
                            open.push(neighbour)
                        },
                    }
            }
        }
        closed.push(node);
    }
}
