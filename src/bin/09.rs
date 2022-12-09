use std::{
    collections::HashSet,
    hash::Hash,
    iter::repeat,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Vec2<T: Add<T, Output = T> + Sub<T, Output = T> + Default + Hash + PartialEq + Eq> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Default + Hash + PartialEq + Eq> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Default + Hash + PartialEq + Eq> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Default + Hash + PartialEq + Eq> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Default + Hash + PartialEq + Eq + Copy> AddAssign
    for Vec2<T>
{
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
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
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

#[derive(Debug)]
struct RopeSim<const N: usize> {
    knots: [Vec2<isize>; N],
}

impl<const N: usize> RopeSim<N> {
    fn new() -> Self {
        Self {
            knots: [Vec2::default(); N],
        }
    }

    fn perform_move(&mut self, mov: &Vec2<isize>) {
        *self.knots.first_mut().unwrap() += *mov;
        for i in 0..self.knots.len() - 1 {
            let head = self.knots[i].clone();
            let tail = &mut self.knots[i + 1];
            // correct tail movement
            let diff = head - *tail;
            if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
                *tail += Vec2::new(diff.x.clamp(-1, 1), diff.y.clamp(-1, 1));
            } else {
                break;
            }
        }
    }
}

fn main() {
    let input = include_str!("../input/09.txt");
    //     let input = "R 5
    // U 8
    // L 8
    // D 3
    // R 17
    // D 10
    // L 25
    // U 20
    // ";
    let mut all_tail_pos2 = HashSet::new();
    let mut all_tail_pos10 = HashSet::new();
    let mut rope_sim: RopeSim<10> = RopeSim::new();
    all_tail_pos2.insert(rope_sim.knots[1]);
    all_tail_pos10.insert(rope_sim.knots[9]);
    for cmd in input.lines().map(Command::parse) {
        for mov in cmd.get_moves() {
            rope_sim.perform_move(&mov);
            all_tail_pos2.insert(rope_sim.knots[1]);
            all_tail_pos10.insert(rope_sim.knots[9]);
        }
    }
    println!("{}", all_tail_pos2.len());
    println!("{}", all_tail_pos10.len());
    // println!("{:?}", all_tail_pos2.iter().map(|p| p.x).max());
    // println!("{:?}", all_tail_pos2.iter().map(|p| p.y).max());
    // println!("{:?}", all_tail_pos2.iter().map(|p| p.x).min());
    // println!("{:?}", all_tail_pos2.iter().map(|p| p.y).min());
}
