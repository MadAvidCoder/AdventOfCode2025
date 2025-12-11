use std::hash::Hash;

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().position(|&c| c == 'S').map(|x| (x as i32, y as i32))
        })
        .expect("No starting point found");

    use std::collections::HashSet;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    fn beam(
        x: i32,
        y: i32,
        grid: &Vec<Vec<char>>,
        visited: &mut HashSet<(i32, i32)>
    ) -> u32 {
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        if x < 0 || x >= width || y < 0 || y >= height {
            return 0;
        }

        if !visited.insert((x, y)) {
            return 0;
        }

        match grid[y as usize][x as usize] {
            '.' | 'S' => {
                beam(x, y+1, grid, visited)
            },
            '^' => {
                beam(x-1, y+1, grid, visited) + beam(x+1, y+1, grid, visited) + 1
            },
            _ => unreachable!(),
        }
    }

    beam(start_x, start_y+1, &grid, &mut visited)
}

pub fn part2(input: &str) -> u32 {
    0
}