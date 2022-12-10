enum Command {
    Noop,
    Addx(isize),
}

fn parse_num_from_bytes(bytes: &[u8]) -> isize {
    let mut b_iter = bytes.iter().peekable();
    let is_negative = match b_iter.peek().cloned() {
        Some(b'-') => {
            let _ = b_iter.next();
            true
        }
        _ => false,
    };
    let mut num = 0;
    for num_byte in b_iter {
        match num_byte {
            b'0'..=b'9' => {
                num = 10 * num + (*num_byte - b'0') as isize;
            }
            _ => break,
        }
    }
    if is_negative {
        -num
    } else {
        num
    }
}

impl Command {
    fn parse(bytes: &[u8]) -> Self {
        match bytes {
            b"noop" => Self::Noop,
            bytes if bytes.starts_with(b"addx ") => Self::Addx(parse_num_from_bytes(&bytes[5..])),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct PositionIter<I: Iterator<Item = Command>> {
    iter: I,
    pending_add: Option<isize>,
    pos: isize,
}

impl<I: Iterator<Item = Command>> PositionIter<I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            pending_add: None,
            pos: 1,
        }
    }
}

// this would be a nice generator...
impl<I: Iterator<Item = Command>> Iterator for PositionIter<I> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(add) = self.pending_add {
            self.pending_add = None;
            let old_pos = self.pos;
            self.pos += add;
            return Some(old_pos);
        }
        match self.iter.next() {
            Some(Command::Noop) => (),
            Some(Command::Addx(add)) => {
                self.pending_add = Some(add);
            }
            None => (),
        }
        Some(self.pos)
    }
}

fn main() {
    let input = include_str!("../input/10.txt");
    // let input = include_str!("../input/10test.txt");
    let cycles_to_check = &[20, 60, 100, 140, 180, 220];
    let mut strength_sum = 0;
    let mut position_iter = PositionIter::new(
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
            .map(Command::parse),
    );
    for (cycle_num, x_reg) in (1..).zip(position_iter.clone()).take(220) {
        if cycles_to_check.binary_search(&cycle_num).is_ok() {
            strength_sum += cycle_num * x_reg;
        }
    }
    println!("{strength_sum}");
    for _ in 0..6 {
        for beam_pos in 0..40 {
            let x_pos = position_iter.next().unwrap();
            if x_pos <= beam_pos + 1 && x_pos >= beam_pos - 1 {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}
