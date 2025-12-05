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
    0
}