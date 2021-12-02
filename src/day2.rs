pub fn solve (input: Vec<String>) {

    //part 1
    solve_part1(&input);
    solve_part2(&input);
}

fn solve_part1 (input: &Vec<String>) {
    let mut depth = 0;
    let mut hpos = 0;

    for s in input {
        if s.starts_with("f") {
            hpos = hpos + s.chars().last().unwrap().to_digit(10).unwrap();
        } else if s.starts_with("u") {
            depth = depth - s.chars().last().unwrap().to_digit(10).unwrap();
        } else if s.starts_with("d") {
            depth = depth + s.chars().last().unwrap().to_digit(10).unwrap();
        }
    }

    println!("Result = {}", depth * hpos);
}

fn solve_part2 (input: &Vec<String>) {
    let mut aim = 0;
    let mut hpos = 0;
    let mut depth = 0;

    for s in input {
        if s.starts_with("f") {
            let x = s.chars().last().unwrap().to_digit(10).unwrap();
            hpos = hpos + x;
            depth = depth + (aim * x);
        } else if s.starts_with("u") {
            aim = aim - s.chars().last().unwrap().to_digit(10).unwrap();
        } else if s.starts_with("d") {
            aim = aim + s.chars().last().unwrap().to_digit(10).unwrap();
        }
    }

    println!("Result = {}", depth * hpos);
}
