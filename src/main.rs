mod util;
mod days;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <day> <part>", args[0]);
        std::process::exit(1);
    }

    let day: u32 = args[1].parse()?;
    let part: u32 = args[2].parse()?;

    let input = util::read_input(day);

    match (day, part) {
        (1, 1) => println!("{}", days::day01::part1(&input)),
        (1, 2) => println!("{}", days::day01::part2(&input)),
        _ => anyhow::bail!("Day {day} part {part} is not implemented"),
    }

    Ok(())
}