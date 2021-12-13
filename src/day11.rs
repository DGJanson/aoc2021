use std::collections::VecDeque;

pub fn solve (input: Vec<String>) {
    let mut field: Field = generate_field(&input);

    /*
    println!("Tests");
    println!("Index 0 neighbours {:?}", field.get_neighbour_indexes(0));
    println!("Index 99 neighbours {:?}", field.get_neighbour_indexes(99));
    println!("Index 10 neighbours {:?}", field.get_neighbour_indexes(10));
    println!("Index 11 neighbours {:?}", field.get_neighbour_indexes(11));
    */

    // part1
    let mut score = 0;
    for _ in 0..100 {
        let mut nines = field.increase_numbers_by_1();
        while !nines.is_empty() {
            nines = field.flash_octopus(nines);
        }
        score = score + field.count_zeros();
    }

    println!("Flashes: {}", score);

    let mut field: Field = generate_field(&input);
    let mut keep_searching = true;
    let mut rounds = 0;

    while keep_searching {
        rounds = rounds + 1;
        let mut nines = field.increase_numbers_by_1();
        while !nines.is_empty() {
            nines = field.flash_octopus(nines);
        }
        if field.count_zeros() == field.length {
            keep_searching = false;
        }
    }

    println!("Rounds until all octopi flash: {}", rounds);
}

struct Field {
    length: usize,
    row_length: usize,
    octopi: Vec<u8>,
}

impl Field {
    fn increase_numbers_by_1 (&mut self) -> VecDeque<usize> {
        let mut nines: VecDeque<usize> = VecDeque::new();
        for i in 0..self.octopi.len() {
            self.octopi[i] = self.octopi[i] + 1;
            if self.octopi[i] > 9 {
                nines.push_back(i);
            }
        }
        nines
    }

    fn flash_octopus (&mut self, mut nines: VecDeque<usize>) -> VecDeque<usize> {
        let index = nines.pop_front().unwrap();
        if self.octopi[index] != 0 {
            self.octopi[index] = 0;

            let neighbours = self.get_neighbour_indexes(index);

            for n in neighbours {
                if self.octopi[n] > 0 {
                    self.octopi[n] = self.octopi[n] + 1;
                    if self.octopi[n] > 9 {
                        nines.push_back(n);
                    }
                }
            }
        }

        nines
    }

    fn get_neighbour_indexes (&self, index: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        if index >= self.row_length {
            result.push(index - self.row_length);
        }
        if index < (self.length - self.row_length) {
            result.push(index + self.row_length);
        }

        if index % self.row_length != 0 { // if 0 leftmost row
            result.push(index - 1);
            if index >= self.row_length {
                result.push(index - self.row_length - 1);
            }
            if index < (self.length - self.row_length) {
                result.push(index + self.row_length - 1);
            }
        }

        if (index + 1) % self.row_length != 0 { // if 0 rightmost row
            result.push(index + 1);
            if index >= self.row_length {
                result.push(index - self.row_length + 1);
            }
            if index < (self.length - self.row_length) {
                result.push(index + self.row_length + 1);
            }
        }

        result
    }

    fn count_zeros (&self) -> usize {
        let mut zeros: usize = 0;
        for n in &self.octopi {
            if *n == 0 {
                zeros = zeros + 1;
            }
        }
        zeros
    }
}

fn generate_field (input: &Vec<String>) -> Field {
    let row_length = input[0].len();
    let mut octopus_vec: Vec<u8> = Vec::new();

    for s in input {
        let chars: Vec<char> = s.chars().collect();
        for c in chars {
            octopus_vec.push(c.to_digit(10).unwrap().try_into().unwrap());
        }
    }

    Field {
        length: octopus_vec.len(),
        row_length: row_length,
        octopi: octopus_vec,
    }
}
