extern crate clap;
extern crate rand;
extern crate ansi_term;

mod node;
mod heuristics;

use node::Node;
use std::collections::{BinaryHeap,HashSet};
use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

fn main() {
    let matches = App::new("Npuzzle")
        .about("Taquin solver")
        .subcommand(SubCommand::with_name("generate")
                    .about("generates random game configuration")
                    .arg(Arg::with_name("size")
                         .short("s")
                         .long("size")
                         .takes_value(true))
                    .arg(Arg::with_name("solvable")
                         .long("solvable")
                         .takes_value(true))
                    .arg(Arg::with_name("iterations")
                         .short("i")
                         .long("iterations")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("solve")
                    .about("solves random game configuration")
                    .arg(Arg::with_name("file")
                         .index(1)
                         .value_name("FILE"))
                    .arg(Arg::with_name("heuristic")
                         .long("heuristic")
                         .short("h")
                         .takes_value(true))).get_matches();

    if let Some(matches) = matches.subcommand_matches("solve") {
        let heuristic = match matches.value_of("heuristic").unwrap_or("manhattan").to_lowercase().as_ref() {
            "manhattan" => heuristics::eval_manhattan,
            "euclide" => heuristics::eval_euclide,
            "conflict" => heuristics::eval_conflict,
            "misplaced" => heuristics::eval_misplaced,
            h  => {
                println!("Invalid value for heuritic: {}, possible values are: \
                \n\tmanhattan: Manhattan distance \
                \n\teuclide: Euclidean distance \
                \n\tlinearconflict: Linear Conflict \
                \n\tmisplaced: Misplaced tiles", h);
                return;
            }
        };

        let file_name = match matches.value_of("file") {
            Some(f) => f,
            _ => {
                println!("Missing map parameter");
                return;
            }
        };
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
            Ok(n) => solve(n, heuristic),
            Err(e) => println!("Error: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let size: usize = matches.value_of("size")
            .unwrap_or("3")
            .parse()
            .unwrap_or_else(|v| {
                println!("#Warning: Invalid value provided for size: {},\
                     usign default value (3)", v);
                3
            });
        let solvable: bool = matches.value_of("solvable")
            .unwrap_or("true")
            .parse()
            .unwrap_or_else(|v| {
                println!("#Warning: Invalid value provided for solvable: {},\
                         usign default value (true)", v);
                true
            });
        let iterations: usize = matches.value_of("iterations")
            .unwrap_or("10")
            .parse()
            .unwrap_or_else(|v| {
                println!("#Warning: Invalid value provided for iterations: {},\
                         using default value (10)", v);
                10
            });

        let n = Node::random(size, iterations, solvable);
        println!("{}", n.len);
        let mut it = n.board.iter().peekable();

        while it.peek().is_some() {
            let line: String = it.by_ref()
                                 .take(n.len)
                                 .map(|v| format!("{0:<3}", v))
                                 .collect::<Vec<_>>()
                                 .join(" ");
            println!("{}", line);
        }
    }
}

fn print_result(n: &Node) -> usize {
    let v = n.parents().collect::<Vec<_>>();
    let mut it = v.iter().rev().map(|node| node.board.clone());
    let v2 = n.parents().collect::<Vec<_>>();
    let it2 = v2.iter().rev().map(|node| node.board.clone());


    let first_node = it.next().unwrap();
    let len = n.len;
    let solution_len = v.len();
    for x in 0..len {
        let line = &first_node[x * len..x * len + len];
        for n in line {
            print!("{} ", ansi_term::Colour::White.paint(format!("{0:<3}", n)));
        }
        println!("");
    }
    println!("");

    for (b1, b2) in it.zip(it2) {
        let colours = Node::format_colors(&b1, &b2);
        for x in 0..len {
            let colored_numbers = colours[x * len..x * len + len].iter().map(|&(c, v)| {
                c.paint(format!("{0:<3}", v.to_string())).to_string()
            }).collect::<Vec<_>>().join(" ");
            println!("{}", colored_numbers);
        }
        println!("");
    }
    println!("Solved in {} moves!", solution_len - 1);

    solution_len
}

pub fn solve(n: Node, h: fn(&node::Node) -> usize) {
    let goal: Node = Node::goal(n.len);

    if n == goal {
        println!("Puzzle is already solved");
        return;
    }

    let r = Rc::new(n);
    let mut open: BinaryHeap<Rc<Node>> = BinaryHeap::new();
    let mut opened: HashSet<Rc<Node>> = HashSet::new();
    let mut closed: HashSet<Rc<Node>> = HashSet::new();

    opened.insert(r.clone());
    open.push(r);
    let mut total_states = 1_usize;
    let mut max_states = 0_usize;

    while let Some(node) = open.pop() {
        let sum = opened.len();
        if sum > max_states {
            max_states = sum;
        }
        if *(node.as_ref()) == goal {
            print_result(node.as_ref());
            println!("{} states ever selected in open set", total_states);
            println!("{} states represented in memory at the same time", max_states);
            break
        } else {
            let r = node;
            let neighbours = Node::get_next_steps(&r, h);

            for neighbour in neighbours {
                if closed.get(&neighbour).is_some() {
                    continue;
                }

                let should_push = opened.get(&neighbour)
                    .map(|node| node.cost > neighbour.cost)
                    .unwrap_or(true);

                if should_push {
                    total_states += 1;
                    let rc = Rc::new(neighbour);
                    opened.insert(rc.clone());
                    open.push(rc.clone());
                }
            }
            opened.remove(r.as_ref());
            closed.insert(r);
        }
    }
}
