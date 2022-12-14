use std::{borrow::Borrow, collections::VecDeque};

use aoc22::{parse_num_from_bytes, Highest};
use nom::{bytes::complete::{tag, take_until1, take_while, take_while1}, IResult, combinator::map_res, character::complete::digit1};


#[derive(Debug, Clone, Copy)]
enum Operation {
    Square,
    Multiply(usize),
    Add(usize)
}

impl Operation {
    fn parse(bytes: &[u8]) -> Self {
        if bytes[0] == b'*' {
            if &bytes[2..5] == b"old" {
                Self::Square
            } else {
                let num = parse_num_from_bytes(&bytes[2..]);
                Self::Multiply(num as usize)
            }
        } else {
            let num = parse_num_from_bytes(&bytes[2..]);
            Self::Add(num as usize)
        }
    }

    fn exec(&self, val: usize) -> usize {
        match self {
            Self::Square => val * val,
            Self::Add(add) => val + add,
            Self::Multiply(mul) => val * mul,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    worry_items: VecDeque<usize>,
    op: Operation,
    div_test: usize,
    monkey_true: usize,
    monkey_false: usize,
    total_items_inspected: usize
}

impl Monkey {
    fn process_next_item(&mut self, divide_by_3: bool) -> Option<(usize, usize)> {
        let item = self.worry_items.pop_front()?;
        self.total_items_inspected += 1;
        let mut new_val = self.op.exec(item);
        if divide_by_3 {
            new_val = new_val / 3;
        }
        if new_val % self.div_test == 0 {
            Some((self.monkey_true, new_val))
        } else {
            Some((self.monkey_false, new_val))
        }
    }
}

fn parse_num_nom(mut input: &[u8]) -> IResult<&[u8], isize> {
    let negative;
    (negative, input) = match tag::<_,_,nom::error::Error<_>>(b"-")(input) {
        Ok((_, input)) => (true, input),
        Err(_) => (false, input),
    };
    let nums;
    (input, nums) = take_while1(|u| matches!(u, b'0'..=b'9'))(input)?;
    let mut num = 0;
    for num_byte in nums {
        match num_byte {
            b'0'..=b'9' => {
                num = 10 * num + <u8 as Into<isize>>::into(*num_byte - b'0');
            }
            _ => break,
        }
    }
    if negative {
        num = -num;
    }
    Ok((input, num))
}

fn main() {
    let input = include_bytes!("../input/11.txt");
    // let input = include_bytes!("../input/11test.txt");
    let (_, mut monkeys) = parse_monkeys(input).unwrap();
    {
        let mut monkeys = monkeys.clone();
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                while let Some((monke, new_val)) = monkeys[i].process_next_item(true) {
                    monkeys[monke].worry_items.push_back(new_val);
                }
            }
        }
        // println!("{:?}", monkeys);
        let mut highest: Highest<2> = Highest::new();
        for monkey in &monkeys {
            highest.insert(monkey.total_items_inspected);
        }
        let [num1, num2] = highest.get();
        println!("{}", num1 * num2);
    }
    // {
    //     for _ in 0..10000 {
    //         for i in 0..monkeys.len() {
    //             while let Some((monke, new_val)) = monkeys[i].process_next_item(false) {
    //                 monkeys[monke].worry_items.push_back(new_val);
    //             }
    //         }
    //     }
    //     // println!("{:?}", monkeys);
    //     let mut highest: Highest<2> = Highest::new();
    //     for monkey in &monkeys {
    //         highest.insert(monkey.total_items_inspected);
    //     }
    //     let [num1, num2] = highest.get();
    //     println!("{}", num1 * num2);
    // }
}

fn parse_monkeys(mut input: &[u8]) -> IResult<&[u8], Vec<Monkey>> {
    let mut monkeys = Vec::new();
    while !input.is_empty() {
        (input, _) = tag(b"Monkey ")(input)?;
        (input, _) = parse_num_nom(input)?;
        (input, _) = tag(b":\n  Starting items: ")(input)?;
        let mut worry_items = VecDeque::new();
        while input[0] != b'\n' {
            if input[0] == b',' {
                input = &input[2..];
            }
            let worry_num;
            (input, worry_num) = parse_num_nom(input)?;
            worry_items.push_back(worry_num as usize);
        }
        (input, _) = tag(b"\n  Operation: new = old ")(input)?;
        let op = Operation::parse(input);
        (input, _) = take_while1(|u| u != b'\n')(input)?;
        (input, _) = tag(b"\n  Test: divisible by ")(input)?;
        let div;
        (input, div) = parse_num_nom(input)?;
        (input, _) = tag(b"\n    If true: throw to monkey ")(input)?;
        let monkey_true;
        (input, monkey_true) = parse_num_nom(input)?;
        (input, _) = tag(b"\n    If false: throw to monkey ")(input)?;
        let monkey_false;
        (input, monkey_false) = parse_num_nom(input)?;
        monkeys.push(Monkey {
            worry_items,
            div_test: div as usize,
            monkey_false: monkey_false as usize,
            monkey_true: monkey_true as usize,
            op,
            total_items_inspected: 0,
        });
        (input, _) = take_while(|u| u == b'\n')(input)?;
    }
    println!("{:?}", monkeys);
    Ok((input, monkeys))
}

