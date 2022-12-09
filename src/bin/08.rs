struct TreeGrid<'a> {
    grid: &'a [u8],
    columns: usize,
    rows: usize,
}

impl<'a> TreeGrid<'a> {
    fn new(grid: &'a [u8]) -> Self {
        let columns = grid.iter().position(|c| *c == b'\n').unwrap_or(0);
        let rows = (1..)
            .find(|c| grid.get((*c * (columns + 1)) as usize).is_none())
            .unwrap();
        TreeGrid {
            grid,
            columns,
            rows,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        let x_u: usize = x.try_into().ok()?;
        let y_u: usize = y.try_into().ok()?;
        if x_u >= self.columns || y_u >= self.rows {
            return None;
        }
        self.grid.get(x_u + y_u * (self.columns + 1)).cloned()
    }
}

fn main() {
    let input = include_str!("../input/08.txt");
    //     let input = "30373
    // 25512
    // 65332
    // 33549
    // 35390
    // ";
    let grid = TreeGrid::new(input.as_bytes());
    println!("{}:{}", grid.columns, grid.rows);
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visible_trees = 0;
    let mut max_scenic_score = 0;
    for x in 0..grid.columns as isize {
        for y in 0..grid.rows as isize {
            // part1
            if directions.iter().any(|dir| {
                // checks if the tree is visible in this direction
                let mut pos = (x, y);
                let start_height = grid.get(pos.0, pos.1).unwrap();
                pos.0 += dir.0;
                pos.1 += dir.1;
                while let Some(height) = grid.get(pos.0, pos.1) {
                    if height >= start_height {
                        return false;
                    }
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                }
                true
            }) {
                visible_trees += 1;
            }
            // part2
            let cur_view_score: usize = directions
                .iter()
                .map(|dir| {
                    // calculate visible direction
                    let mut view_dist = 0;
                    let mut pos = (x, y);
                    let start_height = grid.get(pos.0, pos.1).unwrap();
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                    while let Some(height) = grid.get(pos.0, pos.1) {
                        view_dist += 1;
                        if height >= start_height {
                            break;
                        }
                        pos.0 += dir.0;
                        pos.1 += dir.1;
                    }
                    view_dist
                })
                .product();
            max_scenic_score = max_scenic_score.max(cur_view_score);
        }
    }
    println!("{}", visible_trees);
    println!("{}", max_scenic_score);
}
