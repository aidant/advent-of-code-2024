use std::fs;

use regex::Regex;

pub fn prepare_input() -> Result<String, anyhow::Error> {
    let buf = fs::read("src/day_3_mull_it_over.txt")?;

    let str = String::from_utf8(buf)?;

    Ok(str)
}

pub fn part_1(input: &str) -> Result<u32, anyhow::Error> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total = 0;

    for (_, [lhs, rhs]) in re.captures_iter(input).map(|c| c.extract()) {
        total += lhs.parse::<u32>()? * rhs.parse::<u32>()?
    }

    Ok(total)
}

#[derive(PartialEq, Eq)]
enum Action {
    Do,
    Dont,
}

pub fn part_2(input: &str) -> Result<u32, anyhow::Error> {
    let re1 = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let re2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total = 0;
    let mut action = Action::Do;

    for value in re1.find_iter(input).map(|m| m.as_str()) {
        if value.starts_with("don't") {
            action = Action::Dont
        } else if value.starts_with("do") {
            action = Action::Do
        } else if action == Action::Do {
            for (_, [lhs, rhs]) in re2.captures_iter(value).map(|c| c.extract()) {
                total += lhs.parse::<u32>()? * rhs.parse::<u32>()?
            }
        }
    }

    Ok(total)
}
