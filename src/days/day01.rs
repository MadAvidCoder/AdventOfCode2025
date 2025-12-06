pub fn part1(input: &str) -> i32 {
    let mut loc: i32 = 50;
    let mut code: i32 = 0;
    for line in input.lines() {
        let dir: char = line.chars().next().unwrap();
        let amt: i32 = line[dir.len_utf8()..].parse().unwrap();
        match dir {
            'L' => loc -= amt,
            'R' => loc += amt,
            _ => panic!("{} is invalid", dir),
        }

        if loc < 0 {
            loc += 100;
        } else if loc >= 100 {
            loc -= 100;
        }

        loc = loc % 100;

        if loc == 0 {
            code += 1;
        }
    }
    code
}

pub fn part2(input: &str) -> i32 {
    let mut loc: i32 = 50;
    let mut code: i32 = 0;
    for line in input.lines() {
        let dir: char = line.chars().next().unwrap();
        let amt: i32 = line[dir.len_utf8()..].parse().unwrap();
        let delta: i32 = match dir {
            'L' => -amt,
            'R' => amt,
            _ => panic!("{} is invalid", dir),
        };

        let pos: i32 = loc.rem_euclid(100);

        let mut dist_to_zero = if delta > 0 { (100 - pos) % 100 } else { pos % 100 };
        if dist_to_zero == 0 {
            dist_to_zero = 100;
        }

        if amt >= dist_to_zero {
            code += 1 + (amt - dist_to_zero) / 100;
        }

        loc = (loc + delta).rem_euclid(100);
    }
    code
}