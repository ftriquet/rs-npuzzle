use node;

pub struct Manhattan;
pub struct Euclide;

pub trait Heuristic<T> {
    fn eval(&self, _: T) -> usize {
        0
    }
}

impl Heuristic<node::Node> for Manhattan {
    fn eval(&self, n: node::Node) -> usize {
        let mut sum = 0_usize;
        let goal = node::Node::goal(n.len);

        for val in &n.board {
            let (x, y) = goal.get_pos(*val).unwrap();
            let (pos_x, pos_y) = n.get_pos(*val).unwrap();

            sum += (pos_x as i32 - x as i32).abs() as usize + (pos_y as i32 - y as i32).abs() as usize;
        }

        sum + 1
    }
}

impl Heuristic<node::Node> for Euclide {}
