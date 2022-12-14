use std::iter::Peekable;

#[derive(Debug)]
enum Command<'a> {
    Start,
    EnterDir(&'a str),
    LeaveDir,
    ListDir,
    DirEntry(&'a str),
    FileEntry(&'a str, usize),
}

// (directory sum, total sum)
fn recursive_sum<'a, I: Iterator<Item = Command<'a>>>(
    itr: &mut Peekable<I>,
    all_dirs: &mut Vec<usize>,
) -> usize {
    let mut dir_sum = 0;
    while let Some(cmd) = itr.next() {
        match cmd {
            Command::EnterDir(_) => {
                let subdir_sum = recursive_sum(itr, all_dirs);
                dir_sum += subdir_sum;
                all_dirs.push(subdir_sum);
            }
            Command::LeaveDir => {
                break;
            }
            Command::ListDir => {
                while let Some(list_cmd) = itr.peek() {
                    match list_cmd {
                        Command::DirEntry(_) => (),
                        Command::FileEntry(_, size) => dir_sum += size,
                        _ => break,
                    }
                    let _ = itr.next();
                }
            }
            _ => unreachable!(),
        }
    }
    dir_sum
}

fn main() {
    let input = include_str!("../input/07.txt");
    // let input = include_str!("../input/07test.txt");
    let mut commands = input.lines().map(|line| match line {
        "$ ls" => Command::ListDir,
        "$ cd /" => Command::Start,
        "$ cd .." => Command::LeaveDir,
        line if line.starts_with("$ cd ") => Command::EnterDir(&line[5..]),
        line if line.starts_with("dir ") => Command::DirEntry(&line[4..]),
        line => {
            let (size_s, name) = line.split_once(' ').unwrap();
            let size = size_s.parse().unwrap();
            Command::FileEntry(name, size)
        }
    });
    let _start = commands.next();
    let mut dir_sizes = Vec::new();
    let total_used_space = recursive_sum(&mut commands.clone().peekable(), &mut dir_sizes);
    let sum: usize = dir_sizes.iter().copied().filter(|i| *i < 100_000).sum();
    println!("{}", sum);
    let size_to_free = 30_000_000 - (70_000_000 - total_used_space);
    dir_sizes.sort_unstable();
    // println!("{:?}", dir_sizes);
    let smallest_big_enough = dir_sizes.iter().find(|d| **d > size_to_free);
    println!("{:?}", smallest_big_enough);
}
