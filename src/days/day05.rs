pub fn part1(input: &str) -> u32 {
    let mut count: u32 = 0;

    let parts: Vec<&str> = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let fresh: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|s| {
            let mut nums = s.split('-');
            let start = nums.next().unwrap().parse::<u64>().unwrap();
            let end = nums.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64,u64)>>();
    let available: Vec<u64> = parts[1]
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for a in &available {
        for (start, end) in &fresh {
            if *a >= *start && *a <= *end {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u64 {
    let mut count: u64 = 0;

    let mut fresh: Vec<(u64, u64)> = input
        .split("\r\n\r\n")
        .collect::<Vec<&str>>()[0]
        .lines()
        .map(|s| {
            let mut nums = s.split('-');
            let start = nums.next().unwrap().parse::<u64>().unwrap();
            let end = nums.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64,u64)>>();
    fresh.sort_by_key(|k| k.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in fresh {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    for (start, end) in merged {
        count += end - start + 1;
    }

    count
}