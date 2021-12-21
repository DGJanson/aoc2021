use std::collections::HashMap;

pub fn solve (input: Vec<String>) {

    solve_rounds(&input, 10);
    solve_rounds(&input, 40);

}

fn solve_rounds (input: &Vec<String>, rounds: usize) {
    let mut polymer: HashMap<String, usize> = create_start_polymer(&input[0]);
    // println!("{:#?}", polymer);

    let instructions: HashMap<String, (String, String)> = create_insertions_map(input);
    // println!("{:#?}", instructions);

    for i in 0..rounds {
        println!("Starting round {}", i);
        polymer = do_insertions(polymer, &instructions);
        println!("{:#?}", polymer);

    }
    println!("Score after {} rounds = {}", rounds, calc_score(&polymer));


}

fn do_insertions (polymer: HashMap<String, usize>, instructions: &HashMap<String, (String, String)>) -> HashMap<String, usize> {
    let mut new_polymer: HashMap<String, usize> = HashMap::new();

    for pair in polymer.keys() {
        let number = polymer.get(pair).unwrap();

        if instructions.contains_key(pair) {
            let result_tuple = instructions.get(pair).unwrap();

            add_to_polymer(&mut new_polymer, &result_tuple.0, *number);
            add_to_polymer(&mut new_polymer, &result_tuple.1, *number);
        } else {
            add_to_polymer(&mut new_polymer, pair, *number);
        }
    }

    new_polymer
}

fn add_to_polymer (polymer: &mut HashMap<String, usize>, pair: &String, number: usize) {
    if polymer.contains_key(pair) {
        let existing_number = polymer.get(pair).unwrap();
        polymer.insert(pair.to_string(), *existing_number + number);
    } else {
        polymer.insert(pair.to_string(), number);
    }
}


fn calc_score (polymer: &HashMap<String, usize>) -> usize {
    let mut mappie: HashMap<char, usize> = HashMap::new();

    for c in polymer.keys() {
        let chars: Vec<char> = c.chars().collect();
        let nr = polymer.get(c).unwrap();

        if mappie.contains_key(&chars[0]) {
            let old_value = mappie.get(&chars[0]).unwrap();
            mappie.insert(chars[0], old_value + *nr);
        } else {
            mappie.insert(chars[0], *nr);
        }

        if mappie.contains_key(&chars[1]) {
            let old_value = mappie.get(&chars[1]).unwrap();
            mappie.insert(chars[1], old_value + *nr);
        } else {
            mappie.insert(chars[1], *nr);
        }
    }

    let mut min = usize::MAX;
    let mut max = 0;

    for counts in mappie.values() {
        let mut new_value = *counts;
        if new_value % 2 == 1 {
            new_value = new_value + 1;
        }
        new_value = new_value / 2;
        if new_value < min {
            min = new_value;
        }
        if new_value > max {
            max = new_value;
        }
    }

    max - min
}


fn create_insertions_map (input: &Vec<String>) -> HashMap<String, (String, String)> {
    let mut result: HashMap<String, (String, String)> = HashMap::new();

    for s in input {
        if s.contains("->") {
            let split_input: Vec<&str> = s.split(" -> ").collect();
            let input_chars: Vec<char> = split_input[0].chars().collect();
            let extra_char: char = split_input[1].chars().next().unwrap();

            let first_string = format!("{}{}", input_chars[0], extra_char);
            let second_string = format!("{}{}", extra_char, input_chars[1]);
            result.insert(split_input[0].to_string(), (first_string, second_string));
        }
    }

    result
}

fn create_start_polymer (line: &String) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    for n in 0..(line.len() - 1) {
        let comb = line[n..n+2].to_string();
        if result.contains_key(&comb) {
            let old_value = result.get(&comb).unwrap();
            result.insert(comb, *old_value + 1);
        } else {
            result.insert(comb, 1);
        }
    }

    result
}
