pub fn solve (input: Vec<String>) {
    let nrs: Vec<i32> = parse_input_and_sort(input);

    let median = calc_median(&nrs);

    println!("Median = {}", median);

    let sum = calc_total_fuel(&nrs, median);

    println!("Sum part 1 = {}", sum);

    let avg = calc_average(&nrs);

    println!("Average = {}", avg);

    let sum1 = calc_total_fuel_part2(&nrs, avg);
    let sum2 = calc_total_fuel_part2(&nrs, avg - 1);
    let sum3 = calc_total_fuel_part2(&nrs, avg - 2);

    println!("Sum part 2 = {}", sum1);
    println!("Sum part 2 = {}", sum2);
    println!("Sum part 2 = {}", sum3);

}

fn parse_input_and_sort (input: Vec<String>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::with_capacity(input.len());
    let split_input = input[0].split(",");

    for s in split_input {
        new_vec.push(s.trim().parse().unwrap());
    }

    new_vec.sort();
    new_vec
}

fn calc_median(input: &Vec<i32>) -> i32 {
    let index: usize = input.len() / 2;

    input[index]
}

fn calc_average (input: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for nr in input {
        sum = sum + nr;
    }

    let avg: f64 = (sum as f64) / (input.len() as f64);

    avg.round() as i32
}

fn calc_total_fuel(numbers: &Vec<i32>, hpos: i32) -> i32 {
    let mut total_fuel: i32 = 0;

    for nr in numbers {
        let diff = nr - hpos;
        total_fuel = total_fuel + diff.abs();
    }

    total_fuel
}

fn calc_total_fuel_part2 (numbers: &Vec<i32>, hpos: i32) -> i32 {
    let mut total_fuel: i32 = 0;

    for nr in numbers {
        let diff = nr - hpos;
        total_fuel = total_fuel + calc_part2_diff(diff);
    }

    total_fuel
}

fn calc_part2_diff(diff: i32) -> i32 {
    let abs_diff = diff.abs();

    let mut sum = 0;
    for i in 0..abs_diff {
        sum = sum + i;
    }

    sum + abs_diff
}
