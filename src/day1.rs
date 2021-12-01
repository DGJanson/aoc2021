pub fn solve (measurements: Vec<u16>) {
    let mut prev_measurement = measurements[0];
    let mut total = 0;

    for i in 1..measurements.len() {
        if measurements[i] > prev_measurement {
            total = total + 1;
        }

        prev_measurement = measurements[i];
    }

    println!("Total number of regular increases = {}", total);

    let mut window_measurements: Vec<u16> = Vec::with_capacity(measurements.len() - 2);

    let mut sum1 = measurements[0] + measurements[1];
    let mut sum2 = measurements[1];
    let mut sum3 = 0;

    let mut to_push = 1;

    for i in 2..measurements.len() {
        sum1 = sum1 + measurements[i];
        sum2 = sum2 + measurements[i];
        sum3 = sum3 + measurements[i];

        if to_push == 1 {
            window_measurements.push(sum1);
            sum1 = 0;
            to_push = 2;
        } else if to_push == 2 {
            window_measurements.push(sum2);
            sum2 = 0;
            to_push = 3
        } else if to_push == 3 {
            window_measurements.push(sum3);
            sum3 = 0;
            to_push = 1;
        }
    }

    prev_measurement = window_measurements[0];
    total = 0;

    for i in 1..window_measurements.len() {
        if window_measurements[i] > prev_measurement {
            total = total + 1;
        }

        prev_measurement = window_measurements[i];
    }

    println!("Total number of window increases = {}", total);
}
