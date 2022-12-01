fn main() {
    let input = include_str!("../input/01.txt");
    let mut all_sums = Vec::new();
    let mut cur_sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            all_sums.push(cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<i32>().unwrap();
        }
    }
    all_sums.sort();
    println!("p1: {}", all_sums.last().unwrap());
    let top3sum: i32 = all_sums.iter().rev().take(3).sum();
    println!("p2: {}", top3sum);
}
