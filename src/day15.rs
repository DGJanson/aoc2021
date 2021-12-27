use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve (input: Vec<String>) {
    let square = create_square(&input);

    solve_part1(&square);

    let square = create_huge_square(&input);

    solve_part1(&square);
}

fn solve_part1(square: &Square) {
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut gscore: HashMap<usize, usize> = create_map_with_defaults(square.square.len());
    let mut fscore: HashMap<usize, usize> = create_map_with_defaults(square.square.len());

    gscore.insert(0, 0);
    fscore.insert(0, square.get_heuristic_len(0));

    let mut open_set: HashSet<usize> = HashSet::new();
    open_set.insert(0);

    while !open_set.is_empty() {
        // get current node
        let mut current_score = usize::MAX;
        let mut current_node = 0;
        for n in open_set.iter() {
            if fscore.get(&n).unwrap() < &current_score {
                current_node = *n;
                current_score = *fscore.get(&n).unwrap();
            }
        }

        if current_score != usize::MAX {
            if current_node == square.final_node {
                println!("Found final route. Score = {}", calc_route_score(square, &came_from));
            }
            open_set.remove(&current_node);
            let cur_g_score = *gscore.get(&current_node).unwrap();
            let neighbours = square.get_neighbour_indeces(current_node);
            for nb in neighbours {
                let tent_g_score = cur_g_score + square.get_value(nb);
                if tent_g_score < *gscore.get(&nb).unwrap() {
                    // println!("Found short route from {} to {}", current_node, nb);
                    came_from.insert(nb, current_node);
                    gscore.insert(nb, tent_g_score);
                    fscore.insert(nb, tent_g_score + square.get_heuristic_len(nb));
                    open_set.insert(nb);
                }
            }
        } else {
            println!("Reached dead end, aborting");
            break;
        }
    }
}

struct Square {
    square: Vec<usize>,
    length: usize,
    final_node: usize
}

impl Square {
    fn get_value (&self, index: usize) -> usize {
        if index < self.square.len() {
            return self.square[index];
        }
        panic!("Requested index out of range in square");
    }

    fn get_neighbour_indeces (&self, index: usize) -> Vec<usize> {
        let mut to_return: Vec<usize> = Vec::new();

        if index > self.length {
            to_return.push(index - self.length);
        }

        if index % self.length != 0 {
            to_return.push(index - 1);
        }

        if index < self.square.len() - self.length {
            to_return.push(index + self.length);
        }

        if (index + 1) % self.length != 0 && index < self.square.len() {
            to_return.push(index + 1);
        }

        to_return
    }

    fn get_heuristic_len (&self, index: usize) -> usize {
        let horizontal = self.length - (index % self.length) - 1;
        let vertical = (self.square.len() - 1 - index) / self.length;

        vertical + horizontal
    }
}

fn create_square(input: &Vec<String>) -> Square {
    let length = input[0].len();

    let mut result: Vec<usize> = Vec::new();

    for line in input {
        let chs: Vec<char> = line.chars().collect();
        for c in chs {
            result.push(c.to_digit(10).unwrap().try_into().unwrap());
        }
    }

    Square {
        final_node: result.len() - 1,
        square: result,
        length: length,
    }
}

fn create_huge_square (input: &Vec<String>) -> Square {
    // start with top 10 rows
    let base_length = input[0].len();
    let length = input[0].len() * 5;

    let mut result: Vec<usize> = Vec::new();
    let mut index = 0;

    for n in 0..base_length {
        let chs: Vec<char> = input[n].chars().collect();
        for c in chs {
            result.push(c.to_digit(10).unwrap().try_into().unwrap());
            index = index + 1;
        }
        for _ in 0..4 {
            for i in 0..base_length {
                let new_value = result[index - base_length] + 1;
                if new_value > 9 {
                    result.push(1);
                } else {
                    result.push(new_value);
                }
                index = index + 1;
            }
        }
    }

    let mut index = result.len();
    let lookback = result.len();
    for _ in 0..4 {
        for _ in 0..lookback {
            let new_value = result[index - lookback] + 1;
            if new_value > 9 {
                result.push(1);
            } else {
                result.push(new_value);
            }
            index = index + 1;
        }
    }

    // println!("{:?}", result);

    Square {
        final_node: result.len() - 1,
        square: result,
        length: length
    }
}

fn create_map_with_defaults (len: usize) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in 0..len {
        map.insert(i, usize::MAX);
    }

    map
}

fn calc_route_score (square: &Square, from_map: &HashMap<usize, usize>) -> usize {
    let mut score = 0;
    let mut start_node = false;
    let mut current_node = square.final_node;

    while !start_node {
        if current_node == 0 {
            start_node = true;
        } else {
            score = score + square.get_value(current_node);
            current_node = *from_map.get(&current_node).unwrap();
        }
    }

    score
}
