pub fn part1(input: &str) -> u32 {
    let mut acc: u32 = 0;

    let mut grid: [[u8; 135]; 135] = [[0; 135]; 135];
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '@' {
                grid[r][c] = 1;
            }
        }
    }

    let dirs: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),            (0, 1),
        (1, -1),   (1, 0),  (1, 1),
    ];

    for r in 0..135 {
        for c in 0..135 {
            if grid[r][c] == 0 {
                continue;
            }

            let mut count: u8 = 0;

            for (dr, dc) in dirs {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr < 0 || nr >= 135 || nc < 0 || nc >= 135 {
                    continue;
                }

                count += grid[nr as usize][nc as usize];

                if count >= 4 {
                    break;
                }
            }

            if count < 4 {
                acc += 1;
            }
        }
    }

    acc
}

pub fn part2(input: &str) -> u32 {
    0
}