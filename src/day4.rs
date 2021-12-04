use std::collections::HashMap;

pub fn solve(input: Vec<String>) {

    let mut nrs: Vec<usize> = Vec::new();
    let split_nrs = input[0].split(",");

    for nr in split_nrs {
        nrs.push(nr.parse().unwrap());
    }

    let mut all_cards: Vec<Card> = parse_cards(input);

    'numbers: for nr in nrs {
        for card in &mut all_cards {
            if card.cross_number(nr) { // If true, found card
                println!("Found winning card. Sum = {}", card.sum_remaining());
                println!("Result = {}", card.sum_remaining() * nr);
                break 'numbers;
            }
        }
    }
}

pub fn solve_part2 (input: Vec<String>) {
    let mut nrs: Vec<usize> = Vec::new();
    let split_nrs = input[0].split(",");

    for nr in split_nrs {
        nrs.push(nr.parse().unwrap());
    }

    let mut all_cards: Vec<Card> = parse_cards(input);

    for nr in nrs {
        all_cards = do_round_of_numbers(nr, &mut all_cards);
    }
}

fn do_round_of_numbers (nr: usize, cards: &mut Vec<Card>) -> Vec<Card> {
    let mut new_vec: Vec<Card> = Vec::new();

    let length = cards.len();

    for card in cards {
        if !card.cross_number(nr) {
            new_vec.push(make_new_card(card));            
        } else if length == 1 {
            println!("Sum = {}, nr = {}, total = {}", card.sum_remaining(), nr, nr * card.sum_remaining());
        }
    }
    new_vec
}

struct Card {
    bingo_card: HashMap<usize, (usize, usize)>,
    rows: [usize; 5],
    cols: [usize; 5]
}

fn build_card () -> Card {
    Card {
        bingo_card: HashMap::new(),
        rows: [0; 5],
        cols: [0; 5]
    }
}

fn make_new_card (old_card: &Card) -> Card {
    let mut new_card = Card {
        bingo_card: old_card.bingo_card.clone(),
        rows: [0; 5],
        cols: [0; 5]
    };

    for i in 0..old_card.rows.len() {
        new_card.rows[i] = old_card.rows[i];
    }

    for i in 0..old_card.cols.len() {
        new_card.cols[i] = old_card.cols[i];
    }

    new_card
}

impl Card {
    fn get_total_nrs (&self) -> usize {
        self.bingo_card.len()
    }

    fn add_nr_to_card (&mut self, nr: usize, row: usize, col: usize) {
        self.bingo_card.insert(nr, (row, col));
    }

    fn cross_number (&mut self, nr: usize) -> bool {
        let row_col = self.bingo_card.remove(&nr);
        match row_col {
            Some(rc_tuple) => {
                self.rows[rc_tuple.0] = self.rows[rc_tuple.0] + 1;
                self.cols[rc_tuple.1] = self.cols[rc_tuple.1] + 1;

                if self.rows[rc_tuple.0] == 5 || self.cols[rc_tuple.1] == 5 {
                    true
                } else {
                    false
                }
            }
            None => {
                false
            }
        }
    }

    fn sum_remaining (&self) -> usize {
        let mut total = 0;
        for nr in self.bingo_card.keys() {
            total = total + nr;
        }
        total
    }

}

fn parse_cards (input: Vec<String>) -> Vec<Card> {
    let mut result: Vec<Card> = Vec::new();

    let mut row = 0;
    let mut cur_card = build_card();

    for i in 1..input.len() {
        if input[i].trim() == "" {
            if cur_card.get_total_nrs() == 25 {
                result.push(cur_card);
                cur_card = build_card();
                row = 0;
            }
        } else {
            let split_input = input[i].split_whitespace();
            let mut col = 0;
            for nr in split_input {
                cur_card.add_nr_to_card(nr.trim().parse().unwrap(), row, col);
                col = col + 1;
            }
            row = row + 1;
        }
    }

    if cur_card.get_total_nrs() == 25 {
        result.push(cur_card);
    }

    result
}
