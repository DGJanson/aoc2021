pub fn solve (input: Vec<String>) {
    let mut gol: GameOfLife = build_game_of_life(&input[0]);

    for _ in 0..256 {
        gol.sim_round();
    }

    println!("Day 6 part 1 results: {}", gol.sum_total());
}

struct GameOfLife {
    age_0: usize,
    age_1: usize,
    age_2: usize,
    age_3: usize,
    age_4: usize,
    age_5: usize,
    age_6: usize,
    age_7: usize,
    age_8: usize
}

fn build_game_of_life (input: &String) -> GameOfLife {
    let mut gol = GameOfLife {
        age_0: 0,
        age_1: 0,
        age_2: 0,
        age_3: 0,
        age_4: 0,
        age_5: 0,
        age_6: 0,
        age_7: 0,
        age_8: 0
    };

    let split_input = input.split(",");

    for s in split_input {
        match s.trim() {
            "1" => gol.age_1 = gol.age_1 + 1,
            "2" => gol.age_2 = gol.age_2 + 1,
            "3" => gol.age_3 = gol.age_3 + 1,
            "4" => gol.age_4 = gol.age_4 + 1,
            "5" => gol.age_5 = gol.age_5 + 1,
            "6" => gol.age_6 = gol.age_6 + 1,
            "7" => gol.age_7 = gol.age_7 + 1,
            "8" => gol.age_8 = gol.age_8 + 1,
            other => panic!("Unknown input {}", other),
        }
    }

    gol
}

impl GameOfLife {
    fn sim_round(&mut self) {
        let new_fish = self.age_0;

        self.age_0 = self.age_1;
        self.age_1 = self.age_2;
        self.age_2 = self.age_3;
        self.age_3 = self.age_4;
        self.age_4 = self.age_5;
        self.age_5 = self.age_6;
        self.age_6 = self.age_7;
        self.age_7 = self.age_8;

        self.age_6 = self.age_6 + new_fish;
        self.age_8 = new_fish;
    }

    fn sum_total(&self) -> usize {
        self.age_0 + self.age_1 + self.age_2 + self.age_3 + self.age_4 + self.age_5 + self.age_6 + self.age_7 + self.age_8
    }
}
