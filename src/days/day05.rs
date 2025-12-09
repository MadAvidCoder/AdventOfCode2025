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

pub fn part2(input: &str) -> u32 {
    0
}