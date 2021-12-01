use std::fs;
use std::str::FromStr;

pub fn load_input_and_parse<T>(day: usize, open_test: bool) -> Vec<T>
where T: FromStr, <T as FromStr>::Err : std::fmt::Debug {
    let mut input = "./input";
    if open_test == true {
        input = "./input-test";
    }

    let input_string = fs::read_to_string(format!("{}/{}", input, day)).expect("Cannot open file");

    let rv: Vec<T> = input_string
        .lines()
        .map(|w| w.parse::<T>().unwrap())
        .collect();

    rv
}

pub fn load_input_as_strings(day: usize, open_test: bool) -> Vec<String> {
    let mut input = "./input";
    if open_test == true {
        input = "./input-test";
    }

    let input_string = fs::read_to_string(format!("{}/{}", input, day)).expect("Cannot open file");

    let rv: Vec<String> = input_string
        .lines()
        .map(|s| s.to_string())
        .collect();

    rv
}
