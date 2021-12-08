use std::collections::HashMap;

pub fn solve (input: Vec<String>) {
    let count = count_easy_numbers(&input);

    println!("Part 1 results: {}", count);

    solve_part2(input);

}

fn solve_part2 (input: Vec<String>) {
    let mut total = 0;
    for s in input {
        let first_split = s.split("|");

        let mut first_part = 0;

        let mut input_vec: Vec<&str> = Vec::new();
        let mut output_vec: Vec<&str> = Vec::new();

        for part in first_split {
            if first_part == 0 {
                let second_split = part.split_whitespace();
                for input_string in second_split {
                    input_vec.push(input_string);
                }
                first_part = 1;
            } else {
                let second_split = part.split_whitespace();
                for output_string in second_split {
                    output_vec.push(output_string);
                }
            }
        }

        let result_map = determine_input(input_vec);

        let sum = calc_output(result_map, output_vec);

        total = total + sum;

    }

    println!("Result: {}", total);
}

fn determine_input (input: Vec<&str>) -> HashMap<&str, isize> {
    let mut to_return: HashMap<&str, isize> = HashMap::new();
    // First pass, get the easy values
    let mut one: &str = "";
    let mut four: &str = "";
    let mut seven: &str = "";
    for s in &input {
        let s_len = s.len();
        if s_len == 2 {
            to_return.insert(s, 1);
            one = s;
        } else if s_len == 3 {
            to_return.insert(s, 7);
            seven = s;
        } else if s_len == 4 {
            to_return.insert(s, 4);
            four = s;
        } else if s_len == 7 {
            to_return.insert(s, 8);
        }
    }

    for s in &input {
        if s.len() == 5 && compare_strings(seven, s) == 3 {
            to_return.insert(s, 3);
        } else if s.len() == 5 && compare_strings(four, s) == 3 {
            to_return.insert(s, 5);
        } else if s.len() == 5 {
            to_return.insert(s, 2);
        }
    }

    for s in &input {
        if s.len() == 6 && compare_strings(one, s) == 1 {
            to_return.insert(s, 6);
        } else if s.len() == 6 && compare_strings(four, s) == 3 {
            to_return.insert(s, 0);
        } else if s.len() == 6 {
            to_return.insert(s, 9);
        }
    }

    to_return
}

fn compare_strings (str1: &str, str2: &str) -> usize {
    let second_string: Vec<char> = str2.chars().collect();
    let mut score = 0;

    for ch in str1.chars() {
        if second_string.contains(&ch) {
            score = score + 1;
        }
    }

    score
}

fn calc_output (map: HashMap<&str, isize>, output: Vec<&str>) -> isize {
    let mut total = 0;

    for i in 0..output.len() {
        let s = output[i];
        let mut key: &str = "";

        for k in map.keys() {
            if s.len() == k.len() && compare_strings(s, k) == s.len() {
                key = k;
            }
        }

        let n = map.get(key).unwrap();
        if i == 0 {
            total = total + 1000 * n;
        } else if i == 1 {
            total = total + 100 * n;
        } else if i == 2 {
            total = total + 10 * n;
        } else {
            total = total + n;
        }
    }

    total
}


fn count_easy_numbers(input: &Vec<String>) -> isize{
    let mut sum = 0;

    for s in input {
        let mut first_split = s.split("|");

        let second_split = first_split.nth(1).unwrap().split_whitespace();

        for output in second_split {
            let s_length = output.trim().len();
            if s_length == 2 || s_length == 3 || s_length == 4 || s_length == 7 {
                sum = sum + 1;
            }
        }
    }

    sum
}
