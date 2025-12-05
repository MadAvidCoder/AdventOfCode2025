use std::fs;

pub fn read_input(day: u32) -> String {
    let file = format!("input/day{:02}.txt", day);
    fs::read_to_string(&file).unwrap_or_else(|_| {
        panic!("Failed to read input file: {}", file);
    })
}