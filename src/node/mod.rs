use std::fmt;
use std::cmp::Ordering;
use std::str::FromStr;
use rand;
use rand::Rng;
use std::collections::HashMap;
use ansi_term::Colour;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

type Board = Vec<usize>;

pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive (Clone)]
pub struct Node {
    pub board: Board,
    pub len: usize,
    pub cost: usize,
    pub heuristic: usize,
    pub parents: Option<Rc<Node>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

pub struct NodeIter<'a> {
    current: Option<&'a Node>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        let res = match self.current {
            None => None,
            Some(n) => Some(n),
        };
        match res {
            None => self.current = None,
            Some(r) => {
                self.current = match r.parents {
                    Some(ref r) => Some(r),
                    None => None,
                }
            }
        }
        res
    }
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{{ Board: {:?}({}), Cost: {}, Heuristic: {} }}",
               self.board,
               self.len,
               self.cost,
               self.heuristic)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        if self.board.len() != other.board.len() {
            return false;
        }

        for (i, &v) in self.board.iter().enumerate() {
            if v != other.board[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for Node {}

impl Ord for Node {
    #[cfg(feature = "greedy")]
    fn cmp(&self, other: &Node) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }

    #[cfg(feature = "uniform")]
    fn cmp(&self, other: &Node) -> Ordering {
        (other.heuristic + other.cost).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive (Debug)]
pub enum NodeError {
    ParseError,
    InvalidContentError,
    UnsolvableError,
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeError::ParseError => "invalid game format".fmt(f),
            NodeError::UnsolvableError => "unsolvable game".fmt(f),
            NodeError::InvalidContentError => "invalid board content".fmt(f),
        }
    }
}

fn inversions(board: &[usize]) -> usize {
    let mut res = 0;

    for i in 0..(board.len() - 1) {
        for j in (i + 1)..board.len() {
            if board[i] != 0 && board[j] != 0 && board[i] > board[j]  {
                res += 1;
            }
        }
    }

    res
}

impl FromStr for Node {
    type Err = NodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: Board = Vec::new();
        let mut lines = s.lines().filter_map(|l| {
            let before_comment = l.split('#').next().unwrap_or("").trim();
            if before_comment.is_empty() {
                None
            } else {
                Some(before_comment)
            }
        });

        let len = try!(lines.next()
            .ok_or(NodeError::ParseError)
            .and_then(|line| line.parse::<usize>().map_err(|_| NodeError::ParseError)));

        for l in lines {
            let str_values = l.split_whitespace().collect::<Vec<_>>();
            if str_values.is_empty() {
                continue;
            }

            if str_values.len() != len {
                return Err(NodeError::ParseError);
            }

            let vals_as_nums = str_values.iter().map(|v| v.parse::<usize>());
            for v in vals_as_nums {
                if v.is_err() {
                    return Err(NodeError::ParseError);
                }
                values.push(v.unwrap());
            }
        }

        let node = Node {
            board: values,
            len: len,
            cost: 0,
            heuristic: 0,
            parents: None,
        };
        if !node.is_solvable() {
            Err(NodeError::UnsolvableError)
        } else if node.check_content() {
            Ok(node)
        } else {
            Err(NodeError::InvalidContentError)
        }
    }
}

impl Node {
    pub fn goal(size: usize) -> Node {
        let mut tab: Vec<usize> = vec![0; size * size];
        let mut pos = 0;
        let mut cpt = 0;
        let mut inc = 1_i32;

        for i in 1..size * size {
            tab[pos] = i;

            if cpt + 1 == size || tab[(pos as i32 + inc) as usize] != 0 {
                inc = match inc {
                    1 => size as i32,
                    x if x == size as i32 => -1,
                    -1 => -(size as i32),
                    x if x == -(size as i32) => 1,
                    _ => 0,
                };
                cpt = 1;
            } else {
                cpt += 1;
            }

            pos = (pos as i32 + inc) as usize;
        }

        Node {
            len: size,
            board: tab,
            cost: 0,
            heuristic: 0,
            parents: None,
        }
    }

    pub fn is_solvable(&self) -> bool {
        let goal = Node::goal(self.len);
        let mut goal_invs = inversions(goal.board.as_slice());
        let mut self_invs = inversions(self.board.as_slice());
        if self.len % 2 == 0 {
            self_invs += self.board.iter().position(|&e| e == 0).unwrap_or(0);
            goal_invs += goal.board.iter().position(|&e| e == 0).unwrap_or(0);
        }

        goal_invs % 2 == self_invs % 2
    }

    pub fn check_content(&self) -> bool {
        let mut occurences: HashMap<usize, usize> = HashMap::new();

        for i in 0..self.len * self.len {
            occurences.insert(i, 0);
        }

        for n in &self.board {
            let occ = occurences.entry(*n).or_insert(0);
            *occ += 1;
        }

        for i in 0..self.len * self.len {
            match occurences.get(&i) {
                None => {} // should never happen
                Some(x) => {
                    if *x == 0 {
                        return false;
                    }
                }
            }
        }

        if occurences.len() != self.len * self.len {
            return false;
        }

        true
    }

    pub fn random(size: usize, iterations: usize, solvable: bool) -> Node {
        let mut goal = Self::goal(size);

        for _ in 0..iterations {
            goal.swap_empty();
        }

        if !solvable {
            goal.make_unsolvable();
        }

        goal
    }

    pub fn swap_empty(&mut self) {
        let mut possibilities: Vec<usize> = Vec::new();
        let size = self.len;

        let index = match self.get_pos(0) {
            None => return,
            Some((x, y)) => x * self.len + y,
        };

        if index % size > 0 {
            possibilities.push(index - 1);
        }

        if index % size < size - 1 {
            possibilities.push(index + 1);
        }

        if index / size > 0 {
            possibilities.push(index - size);
        }

        if index / size < size - 1 {
            possibilities.push(index + size);
        }

        let rnd = rand::thread_rng().gen_range::<usize>(0, possibilities.len());
        self.board[index] = self.board[possibilities[rnd]];
        self.board[possibilities[rnd]] = 0;
    }

    pub fn make_unsolvable(&mut self) {
        let (idx1, idx2) = if self.board[0] == 0 || self.board[1] == 0 {
            (self.len * self.len - 1, self.len * self.len - 2)
        } else {
            (0, 1)
        };
        self.board.swap(idx1, idx2);
    }

    pub fn format_colors(b1: &[usize], b2: &[usize]) -> Vec<(Colour, usize)> {
        b1.iter().zip(b2.iter()).map(|(a, b)| {
            if a == b {
                (Colour::White, *a)
            } else {
                (Colour::Green, *a)
            }
        }).collect()
    }

    pub fn print_grid(&self) {
        print!("Board [");
        for x in 0..self.len {
            print!("\n{:?}", &self.board[x * self.len..x * self.len + self.len]);
        }
        println!("] Cost: {}, Heuristic: {}\n", self.cost, self.heuristic);
    }

    pub fn get_array_pos(&self, x: usize, y: usize) -> usize {
        self.len * x + y
    }

    pub fn get_grid_pos(&self, pos: usize) -> (usize, usize) {
        (pos / self.len, pos % self.len)
    }

    pub fn get_pos(&self, num: usize) -> Option<(usize, usize)> {
        match self.board.iter().position(|&r| r == num) {
            Some(pos) => Some((pos / self.len, pos % self.len)),
            None => None,
        }
    }

    fn permute(direction: Direction, h: fn(&Node) -> usize, n: &Rc<Node>) -> Node {
        let (x, y) = n.get_pos(0).unwrap();

        let (new_x, new_y) = match direction {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1),
        };

        let mut new_board = n.board.clone();
        let (pos, new_pos) = (n.get_array_pos(x, y), n.get_array_pos(new_x, new_y));

        new_board.swap(pos, new_pos);

        let tmp_node: Node = Node {
            board: new_board.clone(),
            len: n.len,
            heuristic: 0,
            cost: 0,
            parents: None,
        };

        Node {
            board: new_board,
            len: n.len,
            heuristic: h(&tmp_node),
            cost: n.cost + 1,
            parents: Some(n.clone()),
        }
    }

    pub fn get_next_steps(n: &Rc<Node>, h: fn(&Node) -> usize) -> Vec<Node> {
        let (x, y) = n.get_pos(0).unwrap();

        let mut next_states: Vec<Node> = Vec::with_capacity(4);

        if y != 0 {
            next_states.push(Node::permute(Direction::West, h, n))
        }

        if y != n.len - 1 {
            next_states.push(Node::permute(Direction::East, h, n))
        }

        if x != n.len - 1 {
            next_states.push(Node::permute(Direction::South, h, n))
        }

        if x != 0 {
            next_states.push(Node::permute(Direction::North, h, n))
        }

        next_states
    }

    pub fn parents(&self) -> NodeIter {
        NodeIter {
            current: Some(self),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Node;
    use super::heuristics;
    use super::Direction;
    use std::rc::Rc;
    #[test]
    fn test_permute() {
        let h = heuristics::Manhattan;
        let n: Node = "
3
1   2   3
8   0   4
7   6   5
".parse().expect("Map should be valid");
        let r = Rc::new(n);
        let south = Node::permute(Direction::South, &h, &r);
        assert!(south.board == vec![1, 2, 3, 8, 6, 4, 7, 0, 5]);
        let north = Node::permute(Direction::North, &h, &r);
        assert!(north.board == vec![1, 0, 3, 8, 2, 4, 7, 6, 5]);
        let west = Node::permute(Direction::West, &h, &r);
        assert!(west.board == vec![1, 2, 3, 0, 8, 4, 7, 6, 5]);
        let east = Node::permute(Direction::East, &h, &r);
        assert!(east.board == vec![1, 2, 3, 8, 4, 0, 7, 6, 5]);
    }

    #[test]
    fn parsing_test() {
        let no_size =
"
1   2   3
8   0   4
7   6   5
";
        assert!(no_size.parse::<Node>().is_err());

        let non_numeric_size =
"
hello
1   2   3
8   0   4
7   6   5
";
        assert!(non_numeric_size.parse::<Node>().is_err());

        let inconsistent_size =
"
4
1   2   3
8   0   4
7   6   5
";
        assert!(inconsistent_size.parse::<Node>().is_err());

        let basic_map =
"
3
1   2   3
8   0   4
7   6   5
";
        assert!(basic_map.parse::<Node>().is_ok());
        assert!(basic_map.parse::<Node>().unwrap().board == vec![1, 2, 3, 8, 0, 4, 7, 6, 5]);

        let invalid_content =
"
3
1   2   3
8   0   4
0   6   5
";
        assert!(invalid_content.parse::<Node>().is_err());

        let invalid_content =
"
3
1   2   3
8   0   14
7   6   5
";
        assert!(invalid_content.parse::<Node>().is_err());

        let with_comments =
"
3
# this is a comment
1   2   3
8   0   4 # and another
7   6   5
";
        assert!(with_comments.parse::<Node>().is_ok());
        assert!(with_comments.parse::<Node>().unwrap().board == vec![1, 2, 3, 8, 0, 4, 7, 6, 5]);
    }
}
