use std::collections::VecDeque;

pub fn solve (input: Vec<String>) {
    solve_part1(&input);
}

fn solve_part1(input: &Vec<String>) {
    let mut score = 0;
    for s in input {
        let first_error = find_first_error(s);
        match first_error {
            Some(')') => score = score + 3,
            Some(']') => score = score + 57,
            Some('}') => score = score + 1197,
            Some('>') => score = score + 25137,
            None => score = score + 0,
            _ => println!("Unknown char return"),
        }
    }

    println!("Part 1 results = {}", score);

    let mut score_part2: Vec<usize> = Vec::new();
    for s in input {
        let result = complete_score(s);
        if result > 0 {
            score_part2.push(result);
        }

    }
    score_part2.sort();
    let halfway = score_part2.len() / 2;


    println!("Part 2 results = {}", score_part2[halfway]);
}

fn complete_score (line: &str) -> usize {
    // Repeat the same code as before, but complete correct ones
    let mut round = 0;
    let mut square = 0;
    let mut curly = 0;
    let mut triangle = 0;
    let mut que: VecDeque<char> = VecDeque::new();

    if line == "" {
        0
    } else {
        let chars: Vec<char> = line.chars().collect();
        for c in chars {
            match c {
                '(' => {
                    round = round + 1;
                    que.push_front('(');
                },
                '[' => {
                    square = square + 1;
                    que.push_front('[');
                },
                '{' => {
                    curly = curly + 1;
                    que.push_front('{');
                },
                '<' => {
                    triangle = triangle + 1;
                    que.push_front('<')
                },
                ')' => {
                    if round == 0 {
                        return 0;
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('(') => round = round - 1,
                            _ => {return 0;}
                        }
                    }
                },
                ']' => {
                    if square == 0 {
                        return 0;
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('[') => square = square - 1,
                            _ => {return 0;},
                        }
                    }
                },
                '}' => {
                    if curly == 0 {
                        return 0;
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('{') => curly = curly - 1,
                            _ => {return 0;},
                        }
                    }
                },
                '>' => {
                    if triangle == 0 {
                        return 0;
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('<') => triangle = triangle - 1,
                            _ => {return 0;},
                        }
                    }
                },
                _ => ()
            }
        }
        // Now just find the matching closing arguments to our queueueueueu
        let mut score = 0;
        while !que.is_empty() {
            let pop = que.pop_front();
            match pop {
                Some('(') => score = (score * 5) + 1,
                Some('[') => score = (score * 5) + 2,
                Some('{') => score = (score * 5) + 3,
                Some('<') => score = (score * 5) + 4,
                _ => println!("Unknown character in queue"),
            }
        }
        score
    }
}

fn find_first_error(line: &str) -> Option<char> {
    let mut round = 0;
    let mut square = 0;
    let mut curly = 0;
    let mut triangle = 0;
    let mut que: VecDeque<char> = VecDeque::new();

    if line == "" {
        None
    } else {
        let chars: Vec<char> = line.chars().collect();
        for c in chars {
            match c {
                '(' => {
                    round = round + 1;
                    que.push_front('(');
                },
                '[' => {
                    square = square + 1;
                    que.push_front('[');
                },
                '{' => {
                    curly = curly + 1;
                    que.push_front('{');
                },
                '<' => {
                    triangle = triangle + 1;
                    que.push_front('<')
                },
                ')' => {
                    if round == 0 {
                        return Some(')');
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('(') => round = round - 1,
                            _ => {return Some(')');},
                        }
                    }
                },
                ']' => {
                    if square == 0 {
                        return Some(']');
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('[') => square = square - 1,
                            _ => {return Some(']');},
                        }
                    }
                },
                '}' => {
                    if curly == 0 {
                        return Some('}');
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('{') => curly = curly - 1,
                            _ => {return Some('}');},
                        }
                    }
                },
                '>' => {
                    if triangle == 0 {
                        return Some('>');
                    } else {
                        let pop = que.pop_front();
                        match pop {
                            Some('<') => triangle = triangle - 1,
                            _ => {return Some('>');},
                        }
                    }
                },
                _ => ()
            }
        }
        None
    }
}
