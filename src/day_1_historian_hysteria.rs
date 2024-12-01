use std::fs;

fn prepare_input() -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    let contents_raw = fs::read("src/day_1_historian_hysteria.txt").unwrap();
    let contents_string = String::from_utf8(contents_raw).unwrap();

    for row in contents_string.split('\n') {
        let mut iter = row.trim().splitn(2, ' ').into_iter();
        let l_as_string = iter.next().unwrap().trim();
        let r_as_string = iter.next().unwrap().trim();

        left.push(l_as_string.parse::<i32>().unwrap());
        right.push(r_as_string.parse::<i32>().unwrap());
    }

    (left, right)
}

pub fn day_1_historian_hysteria_part_1() -> i32 {
    let (mut left, mut right) = prepare_input();

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

    total_distance
}

pub fn day_1_historian_hysteria_part_2() -> i32 {
    let (left, right) = prepare_input();

    let mut total_similarity = 0;

    for (i, l) in left.iter().enumerate() {
        let matches = right.iter().filter(|r| l == *r).count();

        total_similarity += l * matches as i32;
    }

    total_similarity
}
