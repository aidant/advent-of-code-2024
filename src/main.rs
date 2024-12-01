#![feature(test)]

mod day_1_historian_hysteria;

fn main() {
    let (day_1_left, day_1_right) = day_1_historian_hysteria::prepare_input().unwrap();
    println!(
        "day 1 part 1: {:?}",
        day_1_historian_hysteria::part_1(&day_1_left, &day_1_right)
    );
    println!(
        "day 1 part 2: {:?}",
        day_1_historian_hysteria::part_2(&day_1_left, &day_1_right)
    );
}
