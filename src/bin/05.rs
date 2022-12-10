use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

#[derive(Clone, Debug)]
struct CrateStacks {
    stacks: Vec<Vec<u8>>,
}

impl CrateStacks {
    pub fn exec_move(&mut self, mov: &Move) {
        for _ in 0..mov.count {
            let from = &mut self.stacks[mov.from - 1];
            let moved = from.pop().unwrap();
            let to = &mut self.stacks[mov.to - 1];
            to.push(moved);
        }
    }

    pub fn exec_order_preserving(&mut self, mov: &Move) {
        let mut tmp = Vec::with_capacity(mov.count);
        let from = &mut self.stacks[mov.from - 1];
        for _ in 0..mov.count {
            tmp.push(from.pop().unwrap());
        }
        let to = &mut self.stacks[mov.to - 1];
        for moved in tmp.into_iter().rev() {
            to.push(moved);
        }
    }
}

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

fn main() {
    let input = include_str!("../input/05.txt");
    let mut stacks = Vec::new();
    let mut is_first_iter = true;
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.starts_with(" 1") {
            break;
        }
        let mut line_chars = line.bytes().peekable();
        let mut cur_stack_idx = 0;
        while let Some(chr) = line_chars.next() {
            if is_first_iter {
                stacks.push(Vec::new());
            }
            // check if crate
            if chr == b'[' {
                stacks[cur_stack_idx].push(line_chars.next().unwrap());
                let _ = line_chars.next();
                let _ = line_chars.next();
            } else {
                // no crate
                let _ = line_chars.next();
                let _ = line_chars.next();
                let _ = line_chars.next();
            }
            cur_stack_idx += 1;
        }
        is_first_iter = false;
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    let _ = lines.next();
    fn parse_move(l: &str) -> IResult<&str, Move> {
        let (l, _) = tag("move ")(l)?;
        let (l, count) = map_res(digit1, str::parse)(l)?;
        let (l, _) = tag(" from ")(l)?;
        let (l, from) = map_res(digit1, str::parse)(l)?;
        let (l, _) = tag(" to ")(l)?;
        let (l, to) = map_res(digit1, str::parse)(l)?;
        Ok((l, Move { from, to, count }))
    }
    // parse moves
    let moves: Vec<_> = lines
        .map(|l| {
            let (_, mov) = parse_move(l).unwrap();
            mov
        })
        .collect();
    let mut stacks = CrateStacks { stacks };
    // part1
    let mut stacks_part1 = stacks.clone();
    for mov in &moves {
        stacks_part1.exec_move(mov);
    }
    let outs: String = stacks_part1
        .stacks
        .iter()
        .filter_map(|v| v.iter().last().map(|u| *u as char))
        .collect();
    println!("{}", outs);
    // part2
    for mov in &moves {
        stacks.exec_order_preserving(mov);
    }
    let outs: String = stacks
        .stacks
        .iter()
        .filter_map(|v| v.iter().last().map(|u| *u as char))
        .collect();
    println!("{}", outs);
}
