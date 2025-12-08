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

pub fn part2(input: &str) -> u64 {
    let mut total_jolts: u64 = 0;

    let banks: Vec<Vec<u32>> =
        input.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

    for bank in banks {
        let mut b = 0;
        for j in (1..=12).rev() {
            let (index, value) = bank[b..bank.len()-j+1]
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
            total_jolts += *value as u64 * 10u64.pow(j as u32 - 1);
            b += index+1;
        }
    }
    total_jolts
}
