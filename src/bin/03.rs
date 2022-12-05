use std::collections::HashSet;

use itertools::Itertools;

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

pub fn first_common_char(s1: &str, s2: &str, s3: &str) -> u8 {
    let s1set = s1.bytes().collect::<HashSet<_>>();
    let s2set = s2.bytes().collect::<HashSet<_>>();
    s3.bytes()
        .find(|b| s1set.contains(b) && s2set.contains(b))
        .unwrap()
}

fn main() {
    let input = include_str!("../input/03.txt");
    let rucksäcke = input.lines().map(|l| {
        let (part1, part2) = l.split_at(l.len() / 2);
        Rucksack { part1, part2 }
    });
    let result: usize = rucksäcke
        .map(|r| {
            let p1chars = r.part1.bytes().collect::<HashSet<_>>();
            let first_common = r.part2.bytes().find(|b| p1chars.contains(b)).unwrap();
            char_prio(first_common) as usize
        })
        .sum();
    println!("{result}");
    let result2: usize = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let first_common = first_common_char(
                chunk.next().unwrap(),
                chunk.next().unwrap(),
                chunk.next().unwrap(),
            );
            char_prio(first_common) as usize
        })
        .sum();
    println!("{result2}");
}
