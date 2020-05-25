const INPUT: usize = 368078;

fn generate_series(series_length: usize) -> Vec<i32> {
    let mut series = Vec::new();

    // Mutable variables to keep track of the series.
    let mut current_value: i32 = 0;
    let mut increasing: bool = true;
    let mut max_value: i32 = 2;
    let mut min_value: i32 = 1;

    series.push(current_value);

    loop {
        // Iterate over all four corners.
        for _ in 0..4 {
            // Within a corner, iterate up and down.
            loop {
                // Mutate the value of `current_value`.
                current_value = if increasing {
                    current_value + 1
                } else {
                    current_value - 1
                };

                // Push the latest value to the series.
                series.push(current_value);

                // If the series is the target length, break.
                if series.len() == series_length {
                    return series;
                }

                // Check if we should flip the sign.
                if current_value == max_value {
                    increasing = false;
                    break;
                } else if current_value == min_value {
                    increasing = true;
                }
            }
        }

        // Update the min and max value.
        max_value += 2;
        min_value += 1;

        // Set the current value one up and start descending.
        current_value += 2;
        increasing = false;
    }
}

fn part_1() -> i32 {
    let series = generate_series(INPUT);

    match series.last() {
        Some(max) => *max,
        None => panic!("Vector is empty"),
    }
}

fn main() {
    println!("{:?}", generate_series(12)); // Use the Debug trait via the format specifier.

    println!("Part 1: {}", part_1());
}
