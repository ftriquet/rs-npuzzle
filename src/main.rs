mod node;
mod heuristics;

use std::collections::BinaryHeap;

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
    };

    solve(n);
}

pub fn solve(n: node::Node) {
    let goal: node::Node = node::Node::goal(n.len);

    let mut open: BinaryHeap<node::Node> = BinaryHeap::new();
    let mut closed: BinaryHeap<node::Node> = BinaryHeap::new();

    open.push(n);

    while let Some(node) = open.pop() {
        if node == goal {
            println!("Solved!");
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
