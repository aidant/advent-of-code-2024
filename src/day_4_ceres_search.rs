use std::{
    fs::File,
    io::{BufRead, BufReader},
    slice::Windows,
    usize,
};

pub fn prepare_input() -> Result<(String, usize), anyhow::Error> {
    let mut result = vec![];

    for line in BufReader::new(File::open("src/day_4_ceres_search.txt")?)
        .lines()
        .map_while(Result::ok)
    {
        result.push(line);
    }

    Ok((result.join(""), result[0].len()))
}

fn get_xy_from_string(
    chars: &[u8],
    width: usize,
    start: usize,
    x_offset: i32,
    y_offset: i32,
) -> u8 {
    let x_base = start % width;
    let y_base = start / width;

    let x = x_base as i32 + x_offset;
    let y = y_base as i32 + y_offset;

    let index = (start as i32 + x_offset + width as i32 * y_offset) as usize;

    if index < chars.len()
        && (0..width).contains(&(x as usize))
        && (0..width).contains(&(y as usize))
    {
        chars[index]
    } else {
        b' '
    }
}

fn get_substring_from_coords(
    chars: &[u8],
    width: usize,
    start: usize,
    coords: &[[i32; 2]],
) -> String {
    String::from_utf8_lossy(
        coords
            .iter()
            .map(|coord| get_xy_from_string(chars, width, start, coord[0].into(), coord[1].into()))
            .collect::<Vec<u8>>()
            .as_slice(),
    )
    .to_string()
}

fn get_substring_from_vector(
    chars: &[u8],
    width: usize,
    start: usize,
    x_vec: i32,
    y_vec: i32,
    length: usize,
) -> String {
    let mut string = "".to_string();

    for i in 0..length {
        let x_offset = i as i32 * x_vec;
        let y_offset = i as i32 * y_vec;

        string.push(get_xy_from_string(chars, width, start, x_offset, y_offset) as char);
    }

    string
}

fn get_substrings_from_all_vectors(
    chars: &[u8],
    width: usize,
    start: usize,
    length: usize,
) -> Vec<String> {
    vec![
        get_substring_from_vector(chars, width, start, 1, 0, length),
        get_substring_from_vector(chars, width, start, 0, 1, length),
        get_substring_from_vector(chars, width, start, 1, 1, length),
        get_substring_from_vector(chars, width, start, -1, 1, length),
        get_substring_from_vector(chars, width, start, -1, 0, length),
        get_substring_from_vector(chars, width, start, 0, -1, length),
        get_substring_from_vector(chars, width, start, -1, -1, length),
        get_substring_from_vector(chars, width, start, 1, -1, length),
    ]
}

pub fn part_1(input: &str, len: usize) -> Result<u32, anyhow::Error> {
    let mut num_of_instances = 0;

    let chars = input.as_bytes();

    for index in 0..chars.len() {
        for substring in get_substrings_from_all_vectors(chars, len, index, 4) {
            if substring == "XMAS" {
                num_of_instances += 1;
            }
        }
    }

    Ok(num_of_instances)
}

pub fn part_2(input: &str, len: usize) -> Result<u32, anyhow::Error> {
    let mut num_of_instances = 0;

    let chars = input.as_bytes();

    for index in 0..chars.len() {
        for substring in vec![
            /*
              M.S
              .A.
              M.S
            */
            get_substring_from_coords(
                chars,
                len,
                index,
                &[[-1, -1], [-1, 1], [0, 0], [1, -1], [1, 1]],
            ),
            /*
              S.S
              .A.
              M.M
            */
            get_substring_from_coords(
                chars,
                len,
                index,
                &[[-1, 1], [1, 1], [0, 0], [-1, -1], [1, -1]],
            ),
            /*
              S.M
              .A.
              S.M
            */
            get_substring_from_coords(
                chars,
                len,
                index,
                &[[1, -1], [1, 1], [0, 0], [-1, -1], [-1, 1]],
            ),
            /*
              M.M
              .A.
              S.S
            */
            get_substring_from_coords(
                chars,
                len,
                index,
                &[[-1, -1], [1, -1], [0, 0], [-1, 1], [1, 1]],
            ),
        ] {
            if substring == "MMASS" {
                num_of_instances += 1;
            }
        }
    }

    Ok(num_of_instances)
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn part_1_example() {
        let example = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";
        let answer = part_1(&example, 10).unwrap();
        assert_eq!(answer, 18);
    }

    #[test]
    fn part_1_test() {
        let (day_4_str, day_4_len) = prepare_input().unwrap();
        let answer = part_1(&day_4_str, day_4_len).unwrap();
        assert_eq!(answer, 2593);
    }

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
        let (day_4_str, day_4_len) = prepare_input().unwrap();
        b.iter(|| part_1(&day_4_str, day_4_len).unwrap())
    }

    #[test]
    fn part_2_example() {
        let example = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";
        let answer = part_2(&example, 10).unwrap();
        assert_eq!(answer, 9);
    }

    #[test]
    fn part_2_test() {
        let (day_4_str, day_4_len) = prepare_input().unwrap();
        let answer = part_2(&day_4_str, day_4_len).unwrap();
        assert_eq!(answer, 1950);
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        let (day_4_str, day_4_len) = prepare_input().unwrap();
        b.iter(|| part_2(&day_4_str, day_4_len).unwrap())
    }
}
