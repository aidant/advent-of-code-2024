use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

use anyhow::anyhow;

pub fn prepare_input() -> Result<(Vec<(u8, u8)>, Vec<Vec<u8>>), anyhow::Error> {
    let mut rules = Vec::<(u8, u8)>::new();
    let mut updates = Vec::<Vec<u8>>::new();

    for line in BufReader::new(File::open("src/day_5_print_queue.txt")?)
        .lines()
        .map_while(Result::ok)
    {
        if line.contains('|') {
            let mut values = line.split('|');

            let l = values.next().ok_or(anyhow!("unable to take left value"))?;
            let r = values.next().ok_or(anyhow!("unable to take right value"))?;

            rules.push((l.parse::<u8>()?, r.parse::<u8>()?));
        }

        if line.contains(',') {
            let values: Vec<&str> = line.split(',').collect();

            let values = values
                .iter()
                .map(|value| value.parse::<u8>())
                .collect::<Result<Vec<u8>, ParseIntError>>()?;

            updates.push(values);
        }
    }

    Ok((rules, updates))
}

fn check_update_against_rules(
    rules: &Vec<(u8, u8)>,
    update: &Vec<u8>,
) -> Result<Option<u8>, anyhow::Error> {
    let mut is_valid = true;

    let rules_before = Vec::<(u8, u8)>::new();
    let rules_after = Vec::<(u8, u8)>::new();

    let middle_index = update.len() / 2;

    for page_index in 0..middle_index {
        for &(a, b) in rules {}
    }

    for page_index in middle_index..update.len() {
        for &(a, b) in rules {}
    }

    Ok(None)
}

pub fn part_1(rules: &Vec<(u8, u8)>, updates: &Vec<Vec<u8>>) -> Result<(), anyhow::Error> {
    for update in updates {
        check_update_against_rules(rules, update)?;
    }

    Ok(())
}
