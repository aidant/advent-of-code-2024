use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn prepare_input() -> Result<Vec<Vec<u8>>, anyhow::Error> {
    let mut result = vec![];

    for line in BufReader::new(File::open("src/day_2_red_nosed_reports.txt")?)
        .lines()
        .map_while(Result::ok)
    {
        let values: Vec<&str> = line.split_whitespace().collect();

        let values = values
            .iter()
            .map(|value| value.parse::<u8>())
            .collect::<Result<Vec<u8>, ParseIntError>>()?;

        result.push(values);
    }

    Ok(result)
}

pub fn is_pair_valid(is_ascending: &mut Option<bool>, a: u8, b: u8) -> bool {
    let delta = if a > b { a - b } else { b - a };

    if !(1..=3).contains(&delta) {
        return false;
    }

    if Option::is_none(is_ascending) {
        *is_ascending = Some(a > b)
    }

    let is_ascending = is_ascending.unwrap();

    if a > b && !is_ascending {
        return false;
    }

    if a < b && is_ascending {
        return false;
    }

    true
}

pub fn is_row_valid(row: &[u8]) -> bool {
    let mut is_ascending: Option<bool> = None;

    for index in 1..row.len() {
        let a = row[index - 1];
        let b = row[index];

        if !is_pair_valid(&mut is_ascending, a, b) {
            return false;
        }
    }

    true
}

fn is_input_valid(
    input: &Vec<Vec<u8>>,
    enable_problem_dampener: bool,
) -> Result<u32, anyhow::Error> {
    let mut safe_count = 0;

    for row in input {
        if is_row_valid(row) {
            safe_count += 1;
        } else if enable_problem_dampener {
            for index in 0..row.len() {
                let mut row = row.to_vec();
                row.remove(index);
                if is_row_valid(&row) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    Ok(safe_count)
}

pub fn part_1(input: &Vec<Vec<u8>>) -> Result<u32, anyhow::Error> {
    is_input_valid(input, false)
}

pub fn part_2(input: &Vec<Vec<u8>>) -> Result<u32, anyhow::Error> {
    is_input_valid(input, true)
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn part_1_test() {
        let day_2 = prepare_input().unwrap();
        let answer = part_1(&day_2).unwrap();
        assert_eq!(answer, 686);
    }

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
        let day_2 = prepare_input().unwrap();
        b.iter(|| part_1(&day_2).unwrap())
    }

    #[test]
    fn part_2_example() {
        let example = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let answer = part_2(&example).unwrap();
        assert_eq!(answer, 4);
    }

    #[test]
    fn part_2_test() {
        let day_2 = prepare_input().unwrap();
        let answer = part_2(&day_2).unwrap();
        assert_eq!(answer, 717);
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        let day_2 = prepare_input().unwrap();
        b.iter(|| part_2(&day_2).unwrap())
    }
}
