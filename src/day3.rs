pub fn solve (input: Vec<String>) {
    solve_part1(&input);
    solve_part2(&input);
}

fn solve_part1 (input: &Vec<String>) {

    let len_input = input[0].len();

    let mut count_array: Vec<u16> = Vec::with_capacity(len_input);
    // Still need to fill vec
    for _ in 0..len_input {
        count_array.push(0);
    }
    let mut total_input = 0; // Count each string

    for s in input {
        total_input = total_input + 1;
        let mut s_chars = s.chars(); // returns iterator
        for i in 0..len_input {
            if s_chars.next().unwrap() == '1' {
                count_array[i] = count_array[i] + 1;
            }
        }
    }

    let cutoff = total_input / 2;
    let mut epsilon: Vec<char> = Vec::with_capacity(len_input);
    let mut gamma: Vec<char> = Vec::with_capacity(len_input);

    for i in count_array {
        if i >= cutoff {
            epsilon.push('1');
            gamma.push('0');
        } else {
            epsilon.push('0');
            gamma.push('1');
        }
    }

    let epsilon_number = isize::from_str_radix(&epsilon.into_iter().collect::<String>(), 2).unwrap();
    let gamma_number = isize::from_str_radix(&gamma.into_iter().collect::<String>(), 2).unwrap();

    println!("Result =  {}", epsilon_number * gamma_number);

}

fn solve_part2 (input: &Vec<String>) {
    let len_input = input[0].len();

    let mut oxy_vec = input.to_vec();

    for i in 0..len_input {
        let most_char = get_most_occurring_bit(&oxy_vec, i, '1');
        oxy_vec = filter_input(oxy_vec, i, most_char);
    }

    let mut co2_vec = input.to_vec();

    for i in 0..len_input {
        let mut most_char = get_most_occurring_bit(&co2_vec, i, '1');
        if most_char == '1' {
            most_char = '0';
        } else {
            most_char = '1';
        }
        co2_vec = filter_input(co2_vec, i, most_char);
        if co2_vec.len() == 1 {
            break;
        }
    }

    let oxy_number = isize::from_str_radix(&oxy_vec[0], 2).unwrap();
    let co2_number = isize::from_str_radix(&co2_vec[0], 2).unwrap();

    println!("Result = {}", oxy_number * co2_number);
}

fn get_most_occurring_bit (input: &Vec<String>, position: usize, bit_char: char) -> char{
    let mut total_input = 0;
    let mut matches = 0;
    for s in input {
        total_input = total_input + 1;
        if s.chars().nth(position).unwrap() == bit_char {
            matches = matches + 1;
        }
    }

    let cutoff = (total_input as f32 / 2.0_f32).ceil() as usize;
    if matches >= cutoff  {
        '1'
    } else {
        '0'
    }
}

fn filter_input (input: Vec<String>, position: usize, select_char: char) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    for s in input {
        if s.chars().nth(position).unwrap() == select_char {
            result.push(s);
        }
    }

    result
}
