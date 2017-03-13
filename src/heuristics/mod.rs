use node;

pub struct Manhattan;
pub struct Euclide;
pub struct LinearConflict;
pub struct MisplacedTiles;

// pub struct Compositor<T, W>(T, W);

// impl<T, W> Heuristic<node::Node> for Compositor<T, W>
// where T: Heuristic<node::Node>,
//       W: Heuristic<node::Node>
// {
//     fn eval(&self, n: &node::Node) -> usize {
//         self.0.eval(n) + self.1.eval(n)
//     }
// }


pub trait Heuristic {
    type T;

    fn eval(&self, _: &Self::T) -> usize {
        0
    }
}

impl Heuristic for Manhattan {
    type T = node::Node;

    fn eval(&self, n: &Self::T) -> usize {
        let mut sum = 0_usize;
        let goal = node::Node::goal(n.len);

        for val in &n.board {
            let (x, y) = goal.get_pos(*val).unwrap();
            let (pos_x, pos_y) = n.get_pos(*val).unwrap();

            sum += (pos_x as i32 - x as i32).abs() as usize + (pos_y as i32 - y as i32).abs() as usize;
        }

        sum
    }
}

impl Heuristic for MisplacedTiles {
    type T = node::Node;

    fn eval(&self, n: &node::Node) -> usize {
        let mut sum = 0_usize;
        let goal = node::Node::goal(n.len);

        for (v, w) in goal.board.iter().zip(n.board.iter()) {
            if v != w {
                sum += 1;
            }
        }

        sum
    }
}

impl Heuristic for Euclide {
    type T = node::Node;

    fn eval(&self, n: &node::Node) -> usize {
        let mut sum = 0_usize;
        let goal = node::Node::goal(n.len);

        for val in &n.board {
            let (x, y) = goal.get_pos(*val).unwrap();
            let (pos_x, pos_y) = n.get_pos(*val).unwrap();
            let dx = (pos_x as i32 - x as i32).abs() as usize;
            let dy = (pos_y as i32 - y as i32).abs() as usize;
            sum += ((dx * dx + dy * dy) as f64).sqrt() as usize;
        }

        sum
    }
}

fn conflict(n: &node::Node, line: usize, col1: usize, col2: usize) -> bool {
    let ivalue = n.board[line * n.len + col1];
    let kvalue = n.board[line * n.len + col2];
    let (ix, iy) = n.get_pos(ivalue).unwrap();
    let (kx, ky) = n.get_pos(kvalue).unwrap();

    ix == kx && iy > ky
}

impl Heuristic for LinearConflict {
    type T = node::Node;

    fn eval(&self, n: &node::Node) -> usize {
        let mut sum = 0_usize;
        for i in 0..n.len {
            for j in 0..n.len {
                for k in j..n.len {
                    if conflict(n, i, j, k) {
                        sum += 2
                    }
                }
            }
        }
        sum
    }
}
