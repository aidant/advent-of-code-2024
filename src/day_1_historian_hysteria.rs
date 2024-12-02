use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;
pub fn prepare_input() -> Result<(Vec<u32>, Vec<u32>), anyhow::Error> {
    let mut left = vec![];
    let mut right = vec![];

    for line in BufReader::new(File::open("src/day_1_historian_hysteria.txt")?)
        .lines()
        .map_while(Result::ok)
    {
        let mut values = line.split_whitespace();

        let l = values.next().ok_or(anyhow!("unable to take left value"))?;
        let r = values.next().ok_or(anyhow!("unable to take right value"))?;

        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    Ok((left, right))
}

pub fn part_1(left: &[u32], right: &[u32]) -> Result<u32, anyhow::Error> {
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

pub fn part_2(left: &[u32], right: &[u32]) -> Result<u32, anyhow::Error> {
    let max = right.iter().max().ok_or(anyhow!("cannot find max value"))?;
    let mut lut = vec![0; *max as usize + 1];

    for &value in right {
        lut[value as usize] += 1;
    }

    let mut total_similarity = 0;

    for &value in left {
        let index = value as usize;

        if index < lut.len() {
            total_similarity += value * lut[index];
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
