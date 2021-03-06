mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {

    // Day 1
    println!("Day 1 results");
    let input = utils::load_input_and_parse::<u16>(1, false);
    day1::solve(input);

    // Day 2
    println!("Day 2 results");
    let input = utils::load_input_as_strings(2, false);
    day2::solve(input);

    // Day 3
    println!("Day 3 results");
    let input = utils::load_input_as_strings(3, false);
    day3::solve(input);

    // Day 4
    println!("Day 4 results");
    let input = utils::load_input_as_strings(4, false);
    day4::solve(input);
    let input = utils::load_input_as_strings(4, false);
    day4::solve_part2(input);

    // Day 5
    println!("Day 5 results");
    let input = utils::load_input_as_strings(5, false);
    day5::solve(input, 1000);

    // Day 6
    println!("Day 6 results");
    let input = utils::load_input_as_strings(6, false);
    day6::solve(input);

    // Day 7
    println!("Day 7 results");
    let input = utils::load_input_as_strings(7, false);
    day7::solve(input);

    // Day 8
    println!("Day 8 results");
    let input = utils::load_input_as_strings(8, false);
    day8::solve(input);

    // Day 9
    println!("Day 9 results");
    let input = utils::load_input_as_strings(9, false);
    day9::solve(input);

    // Day 10
    println!("Day 10 results");
    let input = utils::load_input_as_strings(10, false);
    day10::solve(input);

    // Day 11
    println!("Day 11 results");
    let input = utils::load_input_as_strings(11, false);
    day11::solve(input);

    // Day 12
    println!("Day 12 results");
    let input = utils::load_input_as_strings(12, false);
    day12::solve(input);

    // Day 13
    println!("Day 13 results");
    let input = utils::load_input_as_strings(13, false);
    day13::solve(input);

    // Day 14
    println!("Day 14 results");
    // let input = utils::load_input_as_strings(14, false);
    // day14::solve(input);

    // Day 15
    println!("Day 15 results");
    let input = utils::load_input_as_strings(15, true);
    day15::solve(input);
}
