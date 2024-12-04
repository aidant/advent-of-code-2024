#![feature(test)]

mod day_1_historian_hysteria;
mod day_2_red_nosed_reports;
mod day_3_mull_it_over;
mod day_4_ceres_search;

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
        day_2_red_nosed_reports::part_1(&day_2)
    );
    println!(
        "day 2 part 2: {:?}",
        day_2_red_nosed_reports::part_2(&day_2)
    );

    let day_3 = day_3_mull_it_over::prepare_input().unwrap();
    println!("day 3 part 1: {:?}", day_3_mull_it_over::part_1(&day_3));
    println!("day 3 part 2: {:?}", day_3_mull_it_over::part_2(&day_3));

    let (day_4_str, day_4_len) = day_4_ceres_search::prepare_input().unwrap();
    println!(
        "day 4 part 1: {:?}",
        day_4_ceres_search::part_1(&day_4_str, day_4_len)
    );
    println!(
        "day 4 part 2: {:?}",
        day_4_ceres_search::part_2(&day_4_str, day_4_len)
    );
}
