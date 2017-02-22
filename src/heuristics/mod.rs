use node;

#[derive(Copy, Clone)]
pub struct Manhattan;
#[derive(Copy, Clone)]
pub struct Euclide;

pub trait Heuristic<T>: Copy {
    fn eval(&self, _: T) -> usize {
        0
    }
}

impl Heuristic<node::Node> for Manhattan {
    fn eval(&self, n: node::Node) -> usize {
        let mut sum = 0_usize;
        let goal = node::Node::goal(n.len);

        for (i, val) in n.board.iter().enumerate() {
            let (x, y) = goal.get_pos(*val).unwrap();
            let (pos_x, pos_y) = n.get_pos(*val).unwrap();

            sum += (pos_x as i32 - x as i32).abs() as usize + (pos_y as i32 - y as i32).abs() as usize;
        }

        sum
    }
}

impl Heuristic<node::Node> for Euclide {}
