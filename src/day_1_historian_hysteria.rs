use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

fn prepare_input() -> Result<(Vec<i32>, Vec<i32>), anyhow::Error> {
    let mut left = vec![];
    let mut right = vec![];

    for line in BufReader::new(File::open("src/day_1_historian_hysteria.txt")?)
        .lines()
        .flatten()
    {
        let mut values = line.split_whitespace();

        let l = values.next().ok_or(anyhow!("unable to take left value"))?;
        let r = values.next().ok_or(anyhow!("unable to take right value"))?;

        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }

    Ok((left, right))
}

pub fn day_1_historian_hysteria_part_1() -> Result<i32, anyhow::Error> {
    let (mut left, mut right) = prepare_input()?;

    left.sort();
    right.sort();

    let mut total_distance = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        if l > r {
            total_distance += l - r;
        } else {
            total_distance += r - l;
        }
    }

    Ok(total_distance)
}

pub fn day_1_historian_hysteria_part_2() -> Result<i32, anyhow::Error> {
    let (left, right) = prepare_input()?;

    let mut total_similarity = 0;

    for l in left.iter() {
        let matches = right.iter().filter(|r| l == *r).count();

        total_similarity += l * matches as i32;
    }

    Ok(total_similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let answer = day_1_historian_hysteria_part_1().unwrap();
        assert_eq!(answer, 2066446)
    }

    #[test]
    fn part_2() {
        let answer = day_1_historian_hysteria_part_2().unwrap();
        assert_eq!(answer, 24931009)
    }
}
