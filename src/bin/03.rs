use std::collections::HashSet;

struct Rucksack<'a> {
    part1: &'a str,
    part2: &'a str,
}

pub fn char_prio(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("invalid {c}"),
    }
}

fn main() {
    let input = include_str!("../input/03.txt");
    let rucksäcke = input.lines().map(|l| {
        let (part1, part2) = l.split_at(l.len() / 2);
        Rucksack {part1, part2}
    });
    let result: usize = rucksäcke.map(|r| {
        let p1chars = r.part1.bytes().collect::<HashSet<_>>();
        let mut common = r.part2.bytes().filter(|b| p1chars.contains(&b));
        let first_common = common.next().unwrap();
        char_prio(first_common) as usize
    }).sum();
    println!("{result}");
}
