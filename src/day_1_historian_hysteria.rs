use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

pub fn prepare_input() -> Result<(Vec<i32>, Vec<i32>), anyhow::Error> {
    let mut left = vec![];
    let mut right = vec![];

    for line in BufReader::new(File::open("src/day_1_historian_hysteria.txt")?)
        .lines()
        .map_while(Result::ok)
    {
        let mut values = line.split_whitespace();

        let l = values.next().ok_or(anyhow!("unable to take left value"))?;
        let r = values.next().ok_or(anyhow!("unable to take right value"))?;

        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }

    Ok((left, right))
}

pub fn part_1(left: &Vec<i32>, right: &Vec<i32>) -> Result<i32, anyhow::Error> {
    let mut left = left.to_owned();
    let mut right = right.to_owned();

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

pub fn part_2(left: &Vec<i32>, right: &Vec<i32>) -> Result<i32, anyhow::Error> {
    let mut lut = HashMap::<i32, i32>::new();

    for r in right {
        match lut.get(r) {
            None => lut.insert(r.to_owned(), 1),
            Some(count) => lut.insert(r.to_owned(), count + 1),
        };
    }

    let mut total_similarity = 0;

    for l in left {
        if let Some(count) = lut.get(l) {
            total_similarity += l * count;
        }
    }

    Ok(total_similarity)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn part_1_test() {
        let (left, right) = prepare_input().unwrap();
        let answer = part_1(&left, &right).unwrap();
        assert_eq!(answer, 2066446)
    }

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
        let (left, right) = prepare_input().unwrap();
        b.iter(|| part_1(&left, &right).unwrap())
    }

    #[test]
    fn part_2_test() {
        let (left, right) = prepare_input().unwrap();
        let answer = part_2(&left, &right).unwrap();
        assert_eq!(answer, 24931009)
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        let (left, right) = prepare_input().unwrap();
        b.iter(|| part_2(&left, &right).unwrap())
    }
}
