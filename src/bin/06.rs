fn all_different<T: Eq>(s: &[T]) -> bool {
    for (i, a) in s.iter().enumerate() {
        for b in s.iter().skip(i + 1) {
            if a == b {
                return false;
            }
        }
    }
    true
}

fn main() {
    let input = include_str!("../input/06.txt");
    let all_diff4 = input.as_bytes().windows(4).position(all_different);
    println!("{:?}", all_diff4.map(|d| d + 4));
    let all_diff14 = input.as_bytes().windows(14).position(all_different);
    println!("{:?}", all_diff14.map(|d| d + 14));
}
