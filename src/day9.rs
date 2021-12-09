use std::collections::VecDeque;
use std::collections::HashSet;

pub fn solve (input: Vec<String>) {

    let row_length = input[0].len();
    let array = parse_input(input);

    let part1_answer = solve_part1(&array, row_length);
    let mut part1_score: usize = 0;
    for index in &part1_answer {
        part1_score = part1_score + 1 + (array[*index] as usize);
    }
    println!("Score part 1 = {}", part1_score);

    let part2_score = solve_part2(part1_answer, array, row_length);

    println!("Score part 2 = {}", part2_score);
}

fn solve_part2 (indexes: Vec<usize>, input: Vec<u8>, row_length: usize) -> usize {
    let mut sizes: Vec<usize> = Vec::new();

    for index in indexes {
        let size = calc_size(index, &input, row_length);
        sizes.push(size);
    }

    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

fn calc_size (index: usize, input: &Vec<u8>, row_length: usize) -> usize {
    let mut basin: HashSet<usize> = HashSet::new(); // indexes part of basin
    let mut queue: VecDeque<usize> = VecDeque::new(); // indexes to check
    queue.push_back(index);

    while !queue.is_empty() {
        let to_check = queue.pop_front().unwrap();
        if !basin.contains(&to_check) { // only if not already there!
            basin.insert(to_check);
            let right = to_check + 1;
            if right < input.len() && right % row_length != 0 && input[right] != 9 {
                queue.push_back(right);
            }
            if to_check >= 1 && to_check % row_length != 0 {
                let left = to_check - 1;
                if input[left] != 9 {
                    queue.push_back(left);
                }
            }
            let bottom = to_check + row_length;
            if bottom < input.len() && input[bottom] != 9 {
                queue.push_back(bottom);
            }
            if to_check >= row_length {
                let top = to_check - row_length;
                if input[top] != 9 {
                    queue.push_back(top);
                }
            }
        }
    }

    basin.len()
}

fn solve_part1 (input: &Vec<u8>, row_length: usize) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();

    for i in 0..input.len() {
        let cur_value = input[i];
        let mut right = i + 1;
        let mut left = usize::MAX;
        if i > 0 {
            left = i - 1;
        }
        let mut top = usize::MAX;
        if i >= row_length {
            top = i - row_length;
        }
        let bottom = i + row_length;
        let mut lowest = true;

        // check horizontal boundaries
        if i % row_length == 0 {
            left = usize::MAX;
        }
        if right % row_length == 0 {
            right = usize::MAX;
        }

        if right < input.len() && input[right] <= cur_value {
            lowest = false;
        }
        if lowest && left < input.len() && input[left] <= cur_value {
            lowest = false;
        }
        if lowest && top < input.len() && input[top] <= cur_value {
            lowest = false;
        }
        if lowest && bottom < input.len() && input[bottom] <= cur_value {
            lowest = false;
        }

        if lowest {
            indexes.push(i);
        }
    }

    indexes
}

fn parse_input (input: Vec<String>) -> Vec<u8> {
    let mut to_return: Vec<u8> = Vec::new();

    for s in input {
        let chars: Vec<char> = s.chars().collect();
        for ch in chars {
            let parsed: u8 = ch.to_digit(10).unwrap().try_into().unwrap();
            to_return.push(parsed);
        }
    }

    to_return
}
