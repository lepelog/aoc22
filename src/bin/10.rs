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
        },
        _ => false,
    };
    let mut num = 0;
    for num_byte in b_iter {
        match num_byte {
            b'0'..=b'9' => {
                num = 10 * num + (*num_byte - b'0') as isize;
            },
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
            bytes if bytes.starts_with(b"addx ") => {
                Self::Addx(parse_num_from_bytes(&bytes[5..]))
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = include_str!("../input/10.txt");
    // let input = include_str!("../input/10test.txt");
    let mut cycle_num = 0;
    let cycles_to_check = &[20, 60, 100, 140, 180, 220];
    let mut strength_sum = 0;
    let mut x_reg = 1;
    for cmd in input.as_bytes().split(|b| *b == b'\n').filter(|line| !line.is_empty()).map(Command::parse) {
        match cmd {
            Command::Noop => {
                // takes one cycle, does nothing
                cycle_num += 1;
                if cycles_to_check.binary_search(&cycle_num).is_ok() {
                    println!("{x_reg}:{cycle_num}");
                    strength_sum += cycle_num *  x_reg;
                }
            },
            Command::Addx(add) => {
                // takes 2 cycles
                // tick first cycle
                cycle_num += 1;
                if cycles_to_check.binary_search(&cycle_num).is_ok() {
                    println!("{x_reg}:{cycle_num}");
                    strength_sum += cycle_num * x_reg;
                }
                cycle_num += 1;
                if cycles_to_check.binary_search(&cycle_num).is_ok() {
                    println!("{x_reg}:{cycle_num}");
                    strength_sum += cycle_num *  x_reg;
                }
                x_reg += add;
            }
        }
    }
    println!("{strength_sum}");
}
