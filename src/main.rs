extern crate clap;
extern crate rand;
extern crate ansi_term;

mod node;
mod heuristics;

use node::Node;
use std::collections::BinaryHeap;
use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::Read;

fn main() {
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
        file.read_to_string(&mut s).expect("Unable to read file");

        match s.parse::<Node>() {
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

        let n = Node::random(size, iterations, solvable);
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

fn print_result(n: Node) {
    let p = n.parents.unwrap();
    let mut it = p.iter();
    let it2 = it.clone();
    let first_node = it.next().unwrap();
    let len = n.len;
    for x in 0..len {
        let line = &first_node[x * len..x * len + len];
        for n in line {
            print!("{} ", ansi_term::Colour::White.paint(n.to_string()));
        }
        println!("");
    }
    println!("");

    for (b1, b2) in it.zip(it2) {
        let colours = Node::format_colors(&b1, &b2);
        for x in 0..len {
            let colored_numbers = &colours[x * len..x * len + len].iter().map(|&(c, v)| {
                c.paint(v.to_string()).to_string()
            }).collect::<Vec<_>>();
            for n in colored_numbers {
                print!("{} ", n);
            }
            println!("");
        }
        println!("");
    }
}

pub fn solve(n: Node) {
    let goal: Node = Node::goal(n.len);
    let h = heuristics::Manhattan;

    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed: BinaryHeap<Node> = BinaryHeap::new();

    open.push(n);

    while let Some(node) = open.pop() {
        if node == goal {
            print_result(node);
            println!("Solved!");
            break
        } else {
            let neighbours = node.get_next_steps(&h);

            for neighbour in neighbours {
                if open.iter()
                    .find(|&node| *node == neighbour)
                    .or_else(|| closed.iter().find(|&node| *node == neighbour))
                    .is_none() {
                    open.push(neighbour)
                }
            }
        }
        closed.push(node);
    }
}
