#![feature(test)]
#![feature(let_chains)]

mod day_1_historian_hysteria;
mod day_2_red_nosed_reports;

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

    let day_2 = day_2_red_nosed_reports::prepare_input().unwrap();
    println!(
        "day 2 part 1: {:?}",
        day_2_red_nosed_reports::part_1(day_2.clone())
    );
    println!("day 2 part 2: {:?}", day_2_red_nosed_reports::part_2(day_2));
}
