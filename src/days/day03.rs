pub fn part1(input: &str) -> u32 {
    let mut total_jolts: u32 = 0;

    let banks: Vec<Vec<u32>> =
        input.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

    for bank in banks {
        let (index, value) = bank[ ..bank.len()-1]
            .iter()
            .enumerate()
            .fold(None, |best, (i, v)| {
                match best {
                    None => Some((i, v)),
                    Some((_, bv)) if v > bv => Some((i, v)),
                    Some(o) => Some(o),
                }
            })
            .unwrap();
        total_jolts += value * 10;
        total_jolts += bank[index + 1..]
            .iter()
            .max()
            .unwrap();
    }
    total_jolts
}

pub fn part2(input: &str) -> u32 {
    0
}
