use std::collections::HashSet;

pub fn solve (input: Vec<String>) {

    solve_part1(&input);
    solve_part2(&input);

}

fn solve_part1 (input: &Vec<String>) {
    let mut coords: Vec<Coord> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();

    for line in input {
        if !line.is_empty() && !line.starts_with("fold") {
            coords.push(create_coord(line));
        } else if !line.is_empty() { // fold
            folds.push(create_fold(line));
        }
    }

    let fold = &folds[0];
    for coord in &mut coords {
        if fold.fold_x {
            if fold.nr < coord.x {
                let change = coord.x - fold.nr;
                coord.x = fold.nr - change;
            }
        } else {
            if fold.nr < coord.y {
                let change = coord.y - fold.nr;
                coord.y = fold.nr - change;
            }
        }
    }


    let mut unique_values: HashSet<usize> = HashSet::new();

    for coord in coords {
        unique_values.insert(coord.get_u_value());

    }

    println!("Number of unique coords after 1 fold = {}", unique_values.len());
}

fn solve_part2 (input: &Vec<String>) {
    let mut coords: Vec<Coord> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();

    for line in input {
        if !line.is_empty() && !line.starts_with("fold") {
            coords.push(create_coord(line));
        } else if !line.is_empty() { // fold
            folds.push(create_fold(line));
        }
    }

    for fold in folds {
        for coord in &mut coords {
            if fold.fold_x {
                if fold.nr < coord.x {
                    let change = coord.x - fold.nr;
                    coord.x = fold.nr - change;
                }
            } else {
                if fold.nr < coord.y {
                    let change = coord.y - fold.nr;
                    coord.y = fold.nr - change;
                }
            }
        }
    }


    let mut unique_values: HashSet<usize> = HashSet::new();

    for coord in coords {
        unique_values.insert(coord.get_u_value());
    }
    // base the max number on the folds
    for y in 0..6 {
        for x in 0..40 {
            let coord = y * 1000000 + x;
            if unique_values.contains(&coord) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("\\");
    }

    println!("Number of unique coords after all folds = {}", unique_values.len());
}

struct Coord {
    x: usize,
    y: usize
}

struct Fold {
    fold_x: bool,
    nr: usize
}

fn create_fold (input: &String) -> Fold {

    let is_index = input.find("=").unwrap();
    let mut fold_x = true;
    if input.chars().nth(is_index - 1).unwrap() == 'y' {
        fold_x = false;
    }

    let number:usize = input[is_index + 1..].parse().unwrap();

    Fold {
        fold_x: fold_x,
        nr: number
    }
}

fn create_coord (input: &String) -> Coord {
    let split = input.split(",");

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut first_pass = true;
    for s in split {
        if first_pass {
            x = s.parse().unwrap();
            first_pass = false;
        } else {
            y = s.parse().unwrap();
        }
    }

    Coord {
        x: x,
        y: y
    }
}

impl Coord {
    fn get_u_value (&self) -> usize {
        self.y * 1000000 + self.x
    }
}
