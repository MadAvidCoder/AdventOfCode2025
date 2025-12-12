use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let pair = line
                .split(",")
                .map(|coord| coord.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (pair[0], pair[1])
        })
        .collect();

    let mut row_maxes: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut col_maxes: HashMap<u64, (u64, u64)> = HashMap::new();

    for (x, y) in coords {
        row_maxes
            .entry(y)
            .and_modify(|(min, max)| {
                *min = (*min).min(x);
                *max = (*max).max(x);
            })
            .or_insert((x, x));

        col_maxes
            .entry(x)
            .and_modify(|(min, max)| {
                *min = (*min).min(y);
                *max = (*max).max(y);
            })
            .or_insert((y, y));
    }

    let mut candidates: Vec<(u64, u64)> = Vec::new();

    for (y, (min_x, max_x)) in row_maxes {
        if min_x == max_x {
            candidates.push((min_x, y));;
        } else {
            candidates.push((min_x, y));
            candidates.push((max_x, y));
        }
    }

    for (x, (min_y, max_y)) in col_maxes {
        if min_y == max_y {
            candidates.push((x, min_y));;
        } else {
            candidates.push((x, min_y));
            candidates.push((x, max_y));
        }
    }

    candidates.sort();
    candidates.dedup();

    let mut max_area = 0;

    for (x1, y1) in &candidates {
        for (x2, y2) in &candidates {
            if *x1 == *x2 && *y1 == *y2 {
                continue;
            }

            max_area = max_area.max((x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1));
        }
    }

    max_area
}

pub fn part2(input: &str) -> u32 {
    0
}