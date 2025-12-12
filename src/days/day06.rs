pub fn part1(input: &str) -> u64 {
    let mut total: u64 = 0;

    let sheet: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| {
                    match num.parse::<u32>() {
                        Ok(n) => n,
                        Err(_) => {
                            match num {
                                "+" => 0,
                                "*" => 1,
                                _ => panic!("{} not recognised", num),
                            }
                        },
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let numbers: &[Vec<u32>] = &sheet[..sheet.len()-1];
    let operations: &Vec<u32> = &sheet[sheet.len()-1..][0];

    for (i, n) in operations.iter().enumerate() {
        match n {
            0 => {
                for row in numbers {
                    total += row[i] as u64;
                }
            },
            1 => {
                let mut sub_total: u64 = 0;
                for row in numbers {
                    if sub_total == 0 {
                        sub_total = row[i] as u64;
                    } else {
                        sub_total *= row[i] as u64;
                    }
                }
                total += sub_total;
            },
            _ => panic!("{} unrecognised", n),
        }
    }

    total
}

pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;

    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l
            .chars()
            .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];

    for (r, line) in lines.into_iter().enumerate() {
        for (c, ch) in line.into_iter().enumerate() {
            grid[r][c] = ch;
        }
    }

    let mut problems: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();

    for col in 0..width {
        let is_spaces: bool = (0..height).all(|row| grid[row][col] == ' ');

        if is_spaces {
            if !current.is_empty() {
                problems.push(current.clone());
                current.clear();
            }
        } else {
            current.push(col);
        }
    }

    if !current.is_empty() {
        problems.push(current);
    }

    for problem in problems {
        let add: bool = match grid[grid.len()-1][problem[0]] {
            '+' => true,
            '*' => false,
            _ => panic!("Unknown Operator"),
        };

        let mut numbers: Vec<u64> = Vec::new();

        for col in problem.iter().rev() {
            let mut digits = String::new();
            for row in 0..height-1 {
                let ch = grid[row][*col];
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }
            if !digits.is_empty() {
                numbers.push(digits.parse::<u64>().unwrap());
            }
        }

        if add {
            total += numbers.iter().sum::<u64>();
        } else {
            total += numbers.iter().product::<u64>();
        }
    }

    total
}