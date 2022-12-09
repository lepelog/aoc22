use std::{iter::repeat, ops::{Sub, Add, AddAssign}, hash::Hash, collections::HashSet};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Vec2<T: Add<T, Output=T> + Sub<T, Output=T> + Default + Hash + PartialEq + Eq> {
    x: T,
    y: T
}

impl<T: Add<T, Output=T> + Sub<T, Output=T> + Default + Hash + PartialEq + Eq> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<T, Output=T> + Sub<T, Output=T> + Default + Hash + PartialEq + Eq> Add for Vec2<T> {
    type Output=Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<T, Output=T> + Sub<T, Output=T> + Default + Hash + PartialEq + Eq> Sub for Vec2<T> {
    type Output=Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Add<T, Output=T> + Sub<T, Output=T> + Default + Hash + PartialEq + Eq + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_move(&self) -> Vec2<isize> {
        match self {
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Right => Vec2 { x: 1, y: 0 },
        }
    }
}

struct Command {
    direction: Direction,
    dist: usize,
}

impl Command {
    fn parse(s: &str) -> Self {
        let direction = match s.as_bytes()[0] {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        };

        let dist = s[2..].parse().unwrap();
        Command { direction, dist }
    }

    fn get_moves(&self) -> impl Iterator<Item = Vec2<isize>> {
        repeat(self.direction.get_move()).take(self.dist)
    }
}

#[derive(Debug, Default)]
struct HeadTailSim {
    head: Vec2<isize>,
    tail: Vec2<isize>,
}

impl HeadTailSim {
    fn perform_move(&mut self, mov: &Vec2<isize>) {
        self.head += *mov;
        // correct tail movement
        match self.head - self.tail {
            Vec2 { x: -2, y: _ } => {
                self.tail = self.head + Vec2 { x: 1, y: 0 };
            },
            Vec2 { x: 2, y: _ } => {
                self.tail = self.head - Vec2 { x: 1, y: 0 };
            },
            Vec2 { x: _, y: -2 } => {
                self.tail = self.head + Vec2 { x: 0, y: 1 };
            },
            Vec2 { x: _, y: 2 } => {
                self.tail = self.head - Vec2 { x: 0, y: 1 };
            },
            _ => ()
        }
    }
}

fn main() {
    let input = include_str!("../input/09.txt");
//     let input = "R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2
// ";
    let mut all_tail_pos = HashSet::new();
    let mut sim = HeadTailSim::default();
    all_tail_pos.insert(sim.tail);
    for cmd in input.lines().map(Command::parse) {
        for mov in cmd.get_moves() {
            sim.perform_move(&mov);
            all_tail_pos.insert(sim.tail);
        }
    }
    println!("{}", all_tail_pos.len());
}
