use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve (input: Vec<String>) {
    let hm1 = create_caves(&input);

    solve_part1(hm1);
}

fn solve_part1 (hm: HashMap<String, Cave>) {
    let mut que: VecDeque<String> = VecDeque::new();
    que.push_front("start".to_string());

    println!("Number of start {}", que.iter().filter(|&n| *n == "start".to_string()).count());

    let score = one_dive(&hm, &mut que, 0);

    println!("Paths part 1: {}", score);

    /* !!!! Takes a bit of time....
    let mut que: VecDeque<String> = VecDeque::new();
    que.push_front("start".to_string());

    let score = two_dive(&hm, &mut que, true, 0);

    println!("Paths part 2: {}", score);
    */

}

fn one_dive (hm: &HashMap<String, Cave>, que: &mut VecDeque<String>, mut score: usize) -> usize {
    if !que.is_empty() {

        let cave = hm.get(que.back().unwrap()).unwrap();

        for st in &cave.connections {
            if st == "end" {
                score = score + 1;
            } else {
                let conn_cave = hm.get(st).unwrap();
                if conn_cave.large || !que.contains(&st) { // We can always visit large caves, and small caves not in the queue
                    que.push_back(st.to_string());
                    score = one_dive(hm, que, score);
                }
            }
        }
    }

    que.pop_back();

    score
}

fn two_dive (hm: &HashMap<String, Cave>, que: &mut VecDeque<String>, allow_double: bool, mut score: usize) -> usize {
    if !que.is_empty() {

        let cave = hm.get(que.back().unwrap()).unwrap();

        for st in &cave.connections {
            if st == "end" {
                score = score + 1;
            } else if st != "start" {
                let conn_cave = hm.get(st).unwrap();
                if conn_cave.large {
                    que.push_back(st.to_string());
                    score = two_dive(hm, que, allow_double, score);
                } else {
                    if que.contains(&st) {
                        if allow_double {
                            que.push_back(st.to_string());
                            score = two_dive(hm, que, false, score);
                        }
                    } else {
                        que.push_back(st.to_string());
                        score = two_dive(hm, que, allow_double, score);
                    }
                }
            }
        }
    }

    que.pop_back();

    score
}



struct Cave {
    large: bool,
    connections: Vec<String>,
}

impl Cave {
    fn add_connection (&mut self, cave_name: String) {
        self.connections.push(cave_name);
    }

    fn set_large (&mut self) {
        self.large = true;
    }
}

fn create_caves (input: &Vec<String>) -> HashMap<String, Cave> {
    let mut result: HashMap<String, Cave> = HashMap::new();

    for s in input {
        let split = s.split("-");
        let vec: Vec<&str> = split.collect();
        if !result.contains_key(vec[0].trim()) {
            result.insert(vec[0].trim().to_string(), construct_cave());
            if vec[0].trim().to_uppercase() == vec[0].trim() {
                result.get_mut(vec[0].trim()).unwrap().set_large();
            }
        }

        if !result.contains_key(vec[1].trim()) {
            result.insert(vec[1].trim().to_string(), construct_cave());
            if vec[1].trim().to_uppercase() == vec[1].trim() {
                result.get_mut(vec[1].trim()).unwrap().set_large();
            }
        }

        result.get_mut(vec[0].trim()).unwrap().add_connection(vec[1].trim().to_string());
        result.get_mut(vec[1].trim()).unwrap().add_connection(vec[0].trim().to_string());

    }

    result
}

fn construct_cave () -> Cave {
    let cave = Cave {
        large: false,
        connections: Vec::new(),
    };
    cave
}
