use std::fmt;
use std::cmp::Ordering;
use heuristics;

pub enum Direction {
    North,
    South,
    East,
    West,
}


pub struct Node {
    pub board: Vec<u8>,
    pub len: usize,
    pub cost: usize,
    pub heuristic: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ Board: {:?}({}), Cost: {}, Heuristic: {} }}", self.board, self.len, self.cost, self.heuristic)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        if self.board.len() != other.board.len() {
            return false
        }

        for (i, &v) in self.board.iter().enumerate() {
            if v != other.board[i] {
                return false
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

impl Node {
    pub fn goal(size: usize) -> Node {
        let mut tab: Vec<u8> = vec![0; size * size];
        let mut pos = 0;
        let mut cpt = 0;
        let mut inc = 1_i32;

        for i in 1..size*size {
            tab[pos] = i as u8;

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
        }
    }

    pub fn print_grid(&self) {
        print!("Board [");
        for x in 0..self.len {
            print!("\n{:?}", &self.board[x*self.len .. x*self.len + self.len]);
        }
        println!("] Cost: {}, Heuristic: {}\n", self.cost, self.heuristic);
    }

    pub fn get_array_pos(&self, x: usize, y: usize) -> usize {
        self.len * x + y
    }

    pub fn get_grid_pos(&self, pos: usize) -> (usize, usize) {
        (pos / self.len, pos % self.len)
    }

    pub fn get_pos(&self, num: u8) -> Option<(usize, usize)> {
        match self.board.iter().position(|&r| r == num) {
            Some(pos) => Some((pos / self.len, pos % self.len)),
            None => None
        }
    }

    fn permute(&self, direction: Direction) -> Node {
        let (x, y) = self.get_pos(0).unwrap();

        let (new_x, new_y) = match direction {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1)
        };

        let mut new_board = self.board.clone();
        let tmp: u8;

        let (pos, new_pos) = (self.get_array_pos(x, y), self.get_array_pos(new_x, new_y));

        tmp = new_board[new_pos];
        new_board[new_pos] = 0;
        new_board[pos] = tmp;

        let tmp_node: &Node = &Node {
            board: new_board.clone(),
            len: self.len,
            heuristic: 0,
            cost: 0,
        };

        Node {
            board: new_board,
            len: self.len,
            heuristic: heuristics::eval_heuristic(heuristics::Heuristic::Manhattan, tmp_node),
            cost: self.cost + 1,
        }
    }

    pub fn get_next_steps(&self)  -> Vec<Node> {
        let (x, y) = self.get_pos(0).unwrap();

        let mut next_states: Vec<Node> = Vec::with_capacity(4);

        if x != 0 {
            next_states.push(self.permute(Direction::North))
        }

        if x != self.len - 1 {
            next_states.push(self.permute(Direction::South))
        }

        if y != 0 {
            next_states.push(self.permute(Direction::West))
        }

        if y != self.len - 1 {
            next_states.push(self.permute(Direction::East))
        }

        next_states
    }
}
