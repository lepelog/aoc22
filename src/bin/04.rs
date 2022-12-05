use std::ops::RangeInclusive;

fn parse_range(s: &str) -> RangeInclusive<usize> {
    let (n1, n2) = s.split_once('-').unwrap();
    n1.parse().unwrap()..=n2.parse().unwrap()
}

fn parse_ranges(s: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (range1, range2) = s.split_once(',').unwrap();
    (parse_range(range1), parse_range(range2))
}

fn includes(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r1.start() <= r2.start() && r1.end() >= r2.end()
}

fn overlap(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    !(r1.start() > r2.end() || r1.end() < r2.start())
}

fn main() {
    let input = include_str!("../input/04.txt");
    let totally_included = input
        .lines()
        .map(parse_ranges)
        .filter(|(r1, r2)| includes(r1, r2) || includes(r2, r1))
        .count();
    println!("{totally_included}");
    let total_overlap = input
        .lines()
        .map(parse_ranges)
        .filter(|(r1, r2)| overlap(r1, r2))
        .count();
    println!("{total_overlap}");
}
