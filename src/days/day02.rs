pub fn part1(input: &str) -> u64 {
    let mut ranges: Vec<(u64, u64)> = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut nums = part.split('-');
            let start = nums.next().unwrap().parse::<u64>().unwrap();
            let end = nums.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect();
    ranges.sort_by_key(|k| k.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    let mut invalid_sum: u64 = 0;

    for (start, end) in merged {
        let mut h: u64 = 1;

        loop {
            let p = 10u64.pow(h.to_string().len() as u32);
            let id = h * p + h;

            if id > end {
                break;
            }

            if id >= start {
                invalid_sum += id;
            }

            h += 1;
        }

    }
    invalid_sum
}

pub fn part2(input: &str) -> u64 {
    use std::collections::HashSet;
    let mut invalid_ids: HashSet<u64> = HashSet::new();

    let mut ranges: Vec<(u64, u64)> = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut nums = part.split('-');
            let start = nums.next().unwrap().parse::<u64>().unwrap();
            let end = nums.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect();
    ranges.sort_by_key(|k| k.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    let mut invalid_sum: u64 = 0;

    for (start, end) in merged {
        let max_digits = end.to_string().len();

        for d in 1..=(max_digits / 2) {
            let p_min = 10u64.pow(d as u32 - 1);
            let p_max = 10u64.pow(d as u32) - 1;

            for k in 2..=max_digits / d {
                let total_digits = d * k;
                if total_digits > max_digits {
                    break;
                }

                for p in p_min..=p_max {
                    let mut id: u64 = 0;
                    for _ in 0..k {
                        id = id * 10u64.pow(d as u32) + p;
                    }
                    if id > end {
                        break;
                    }

                    if id >= start && invalid_ids.insert(id) {
                        invalid_sum += id;
                    }
                }
            }
        }
    }
    invalid_sum
}