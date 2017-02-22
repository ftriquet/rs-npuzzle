use std::fmt;
use std::cmp::Ordering;
use heuristics;
use std::str::FromStr;
use rand;
use rand::Rng;
use std::collections::HashMap;

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
    pub parents: Option<Vec<Board>>,
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
        other.heuristic.cmp(&(self.cost + self.heuristic))
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

impl FromStr for Node {
    type Err = NodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: Board = Vec::new();
        let mut lines = s.lines();

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

        if node.check_content() {
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

    fn permute<H: heuristics::Heuristic<Node>>(&self, direction: Direction, h: &H) -> Node {
        let (x, y) = self.get_pos(0).unwrap();

        let (new_x, new_y) = match direction {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1),
        };

        let mut new_board = self.board.clone();
        let tmp: usize;

        let (pos, new_pos) = (self.get_array_pos(x, y), self.get_array_pos(new_x, new_y));

        tmp = new_board[new_pos];
        new_board[new_pos] = 0;
        new_board[pos] = tmp;

        let tmp_node: Node = Node {
            board: new_board.clone(),
            len: self.len,
            heuristic: 0,
            cost: 0,
            parents: None,
        };

        let mut parents = match self.parents {
            Some(ref p) => p.clone(),
            None => Vec::new(),
        };

        parents.push(self.board.clone());

        Node {
            board: new_board,
            len: self.len,
            heuristic: h.eval(tmp_node),
            cost: self.cost + 1,
            parents: Some(parents),
        }
    }

    pub fn get_next_steps<H: heuristics::Heuristic<Node>>(&self, h: &H) -> Vec<Node> {
        let (x, y) = self.get_pos(0).unwrap();

        let mut next_states: Vec<Node> = Vec::with_capacity(4);

        if x != 0 {
            next_states.push(self.permute(Direction::North, h))
        }

        if x != self.len - 1 {
            next_states.push(self.permute(Direction::South, h))
        }

        if y != 0 {
            next_states.push(self.permute(Direction::West, h))
        }

        if y != self.len - 1 {
            next_states.push(self.permute(Direction::East, h))
        }

        next_states
    }
}
