pub fn solve (input: Vec<String>, length: usize) {
    let mut grid = create_grid(length);

    grid = read_lines(input, grid, false);

    let score = calculate_score(grid);
    println!("Day 5 part 1 result {}", score)
}

fn read_lines (input: Vec<String>, mut grid: Vec<Vec<usize>>, only_straight: bool) -> Vec<Vec<usize>> {
    for s in input {
        let coords = parse_line(s);
        if !only_straight || (coords[0] == coords[2] || coords[1] == coords[3]) {
            grid = cross_coords(coords, grid);
        }
    }

    grid
}

fn cross_coords (coords: [isize; 4], mut grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let difx: isize = coords[2] - coords[0];
    let dify: isize = coords[3] - coords[1];

    let lx: usize = coords[2].try_into().unwrap();
    let ly: usize = coords[3].try_into().unwrap();
    grid[ly][lx] = grid[ly][lx] + 1;

    if difx == 0 {
        if dify > 0 {
            for i in 0..dify {
                let cx: usize = (coords[0]).try_into().unwrap();
                let cy: usize = (coords[1] + i).try_into().unwrap();
                grid[cy][cx] = grid[cy][cx] + 1;
            }
        } else {
            for i in (dify + 1)..1{
                let cx: usize = (coords[0]).try_into().unwrap();
                let cy: usize = (coords[1] + i).try_into().unwrap();
                grid[cy][cx] = grid[cy][cx] + 1;
            }
        }
    } else if dify == 0 {
        if difx > 0 {
            for i in 0..difx {
                let cx: usize = (coords[0] + i).try_into().unwrap();
                let cy: usize = (coords[1]).try_into().unwrap();
                grid[cy][cx] = grid[cy][cx] + 1;
            }
        } else {
            for i in (difx + 1)..1 {
                let cx: usize = (coords[0] + i).try_into().unwrap();
                let cy: usize = (coords[1]).try_into().unwrap();
                grid[cy][cx] = grid[cy][cx] + 1;
            }
        }
    } else { // always diagonal 45 degrees
        let steps = difx.abs();
        let mut xdir: isize = 1;
        if difx < 0 {
            xdir = -1;
        }
        let mut ydir: isize = 1;
        if dify < 0 {
            ydir = -1;
        }

        for i in 0..steps {
            let cx: usize = (coords[0] + (i * xdir)).try_into().unwrap();
            let cy: usize = (coords[1] + (i * ydir)).try_into().unwrap();
            grid[cy][cx] = grid[cy][cx] + 1;
        }
    }

    grid
}

fn calculate_score (grid: Vec<Vec<usize>>) -> usize {
    let mut score: usize = 0;
    for row in grid {
        for value in row {
            if value >= 2 {
                score = score + 1;
            }
        }
    }

    score
}

fn create_grid(length: usize) -> Vec<Vec<usize>> {
    let mut main_vec: Vec<Vec<usize>> = Vec::with_capacity(length);

    for _ in 0..length {
        let row_vec: Vec<usize> = vec![0; length];
        main_vec.push(row_vec);
    }

    main_vec
}

fn parse_line (input: String) -> [isize; 4] {
    let mut to_return = [0; 4];

    let split_input = input.split(" -> ");
    let mut indexer = 0;
    for s in split_input {
        let split_split_input = s.split(",");
        for ss in split_split_input {
            to_return[indexer] = ss.parse().unwrap();
            indexer = indexer + 1;
        }
    }

    to_return
}
