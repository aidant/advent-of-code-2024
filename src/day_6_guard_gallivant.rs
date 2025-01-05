use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

use crate::day_4_ceres_search::get_index_from_string;

pub fn prepare_input() -> Result<(String, usize), anyhow::Error> {
    let mut result = vec![];

    for line in BufReader::new(File::open("src/day_6_guard_gallivant.txt")?)
        .lines()
        .map_while(Result::ok)
    {
        result.push(line);
    }

    Ok((result.join(""), result[0].len()))
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<(i32, i32)> for Direction {
    fn into(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl Direction {
    pub fn rotate_cw(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub fn part_1(input: &str, width: usize) -> Result<u32, anyhow::Error> {
    let mut distinct_positions = HashMap::<usize, ()>::new();
    let chars = input.as_bytes();
    let mut chars_history = chars.clone().to_owned();

    let mut direction = Direction::Up;
    let mut index = input.find('^');

    loop {
        if let Some(current_index) = index {
            distinct_positions.insert(current_index, ());

            let (x_offset, y_offset) = direction.into();
            let new_index = get_index_from_string(chars, width, current_index, x_offset, y_offset);

            if let Some(new_index) = new_index {
                if chars[new_index] == b'#' {
                    direction = direction.rotate_cw();
                }
            }

            chars_history[current_index] = b'X';

            println!("{}, {:?}", chars[current_index] as char, direction);

            let (x_offset, y_offset) = direction.into();
            index = get_index_from_string(chars, width, current_index, x_offset, y_offset);
        } else {
            break;
        }
    }

    println!(
        "{:?}",
        String::from_utf8(chars_history)
            .unwrap()
            .as_bytes()
            .chunks(width)
            .map(|c| c.iter().map(|c| *c as char).collect())
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(distinct_positions.len() as u32)
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn part_1_example() {
        let example = "....#..............#............#..............#.............#..^.............#.#...............#...";
        let answer = part_1(&example, 10).unwrap();
        assert_eq!(answer, 41);
    }
}
