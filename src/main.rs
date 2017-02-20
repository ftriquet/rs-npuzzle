extern crate clap;
extern crate rand;

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
        .subcommand(SubCommand::with_name("generate")
                    .about("generates random game configuration")
                    .arg(Arg::with_name("size")
                         .short("s")
                         .long("size")
                         .value_name("SIZE"))
                    .arg(Arg::with_name("solvable")
                         .long("solvable")
                         .value_name("SOLVABLE"))
                    .arg(Arg::with_name("iterations")
                         .short("i")
                         .long("iterations")
                         .value_name("ITERATIONS")))
        .subcommand(SubCommand::with_name("solve")
                    .about("solves random game configuration")
                    .arg(Arg::with_name("file")
                         .index(1)
                         .value_name("FILE"))).get_matches();

    if let Some(matches) = matches.subcommand_matches("solve") {

        let file_name = matches.value_of("file").unwrap_or("default.map");
        let mut file = match File::open(file_name) {
            Ok(f) => f,
            Err(e) => {
                println!("Error: {}: {}", file_name, e);
                return;
            }
        };

        let mut s = String::new();
        file.read_to_string(&mut s);

        match s.parse::<node::Node>() {
            Ok(n) => solve(n),
            Err(e) => println!("Error: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let size: usize = matches.value_of("size").unwrap_or("3").parse().unwrap_or_else(|v| {
            println!("Warning: Invalid value provided for size: {}, usign default value (3)", v);
            3
        });
        let solvable: bool = matches.value_of("solvable").unwrap_or("true").parse().unwrap_or_else(|v| {
            println!("Warning: Invalid value provided for solvable: {}, usign default value (true)", v);
            true
        });
        let iterations: usize = matches.value_of("iterations").unwrap_or("10").parse().unwrap_or_else(|v| {
            println!("Warning: Invalid value provided for iterations: {}, usign default value (10)", v);
            10
        });

        let n = node::Node::random(size, iterations, solvable);
        println!("{}", n.len);
        let mut it = n.board.iter().peekable();

        while it.peek().is_some() {
            let line: String = it.by_ref()
                                 .take(n.len)
                                 .map(|v| format!("{0:-2}", v))
                                 .collect::<Vec<_>>()
                                 .join(" ");
            println!("{}", line);
        }
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
