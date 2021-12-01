mod utils;

fn main() {
    let input = utils::load_input_as_strings(1, true);

    for number in input {
        println!("{}", number);
    }
}
