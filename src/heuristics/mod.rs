use node;

pub enum Heuristic {
    Manhattan,
    Euclide,
}

pub fn eval_heuristic(h: Heuristic, node: &node::Node) -> usize {
    match h {
        Heuristic::Manhattan => {
            manhattan(node) + linear_conflict(node)
        },
        Heuristic::Euclide => {
            0
        }
    }
}

pub fn manhattan(node: &node::Node) -> usize {
    let mut sum = 0_usize;
    let goal = node::Node::goal(node.len);

    for (i, val) in node.board.iter().enumerate() {
        let (x, y) = goal.get_pos(*val).unwrap();
        let (pos_x, pos_y) = node.get_pos(*val).unwrap();

        sum +=
            (pos_x as i32 - x as i32).abs() as usize +
            (pos_y as i32 - y as i32).abs() as usize;
    }

    sum
}

pub fn conflict(node: &node::Node, i: usize, j: usize, k: usize) -> bool {
    let v1 = node.board[i * node.len + j];
    let v2 = node.board[i * node.len + k];
    let goal = node::Node::goal(node.len);

    let (x1, y1) = goal.get_pos(v1).unwrap();
    let (x2, y2) = goal.get_pos(v2).unwrap();

    x1 == x2 && y1 > y2
}

pub fn linear_conflict(node: &node::Node) -> usize {
    let mut sum = 0_usize;

    for i in 0..node.len {
        for j in 0..node.len {
            for k in j+1..node.len {
                if conflict(node, i, j, k) {
                    sum += 2;
                }
            }
        }
    }

    sum
}
