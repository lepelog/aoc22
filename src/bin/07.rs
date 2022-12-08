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
fn recursive_sum<'a, I: Iterator<Item = Command<'a>>>(itr: &mut Peekable<I>, all_dirs: &mut Vec<usize>) -> (usize, usize) {
    let mut total_sum = 0;
    let mut dir_sum = 0;
    while let Some(cmd) = itr.next() {
        match cmd {
            Command::EnterDir(_) => {
                let (subdir_sum, subtotal) = recursive_sum(itr, all_dirs);
                total_sum += subtotal;
                dir_sum += subdir_sum;
                all_dirs.push(subdir_sum);
                if subdir_sum < 100000 {
                    total_sum += subdir_sum;
                }
            },
            Command::LeaveDir => {
                break;
            },
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
    (dir_sum, total_sum)
}

fn main() {
    let input = include_str!("../input/07.txt");
    // let input = include_str!("../input/07test.txt");
    let mut commands = input.lines().map(|line| {
        match line {
            "$ ls" => Command::ListDir,
            "$ cd /" => Command::Start,
            "$ cd .." => Command::LeaveDir,
            line if line.starts_with("$ cd ") => Command::EnterDir(&line[5..]),
            line if line.starts_with("dir ") => Command::DirEntry(&line[4..]),
            line => {
                let (size_s, name) = line.split_once(' ').unwrap();
                let size = size_s.parse().unwrap();
                Command::FileEntry(name, size)
            },
        }
    });
    let _start = commands.next();
    let mut v = Vec::new();
    let (total_used_space, total_small_sums) = recursive_sum(&mut commands.clone().peekable(), &mut v);
    println!("{}",total_small_sums);
    let sum: usize = v.iter().cloned().filter(|i| *i < 100000).sum();
    println!("{}", sum);
    // let size_to_free = 30000000 - (70000000 - total_used_space);
    // println!("{}", size_to_free);
}
