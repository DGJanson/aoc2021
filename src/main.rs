mod utils;

mod day1;
mod day2;

fn main() {
    // Day 1
    println!("Day 1 results");
    let input = utils::load_input_and_parse::<u16>(1, false);
    day1::solve(input);

    // Day 2
    println!("Day 2 results");
    let input = utils::load_input_as_strings(2, false);
    day2::solve(input);

}
