mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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

}
