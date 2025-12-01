use std::fs;

fn main() {
    // .expect() handles the error by crashing with a custom message if it fails
    let data = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");


    let deltas: Vec<i32> = data
        .split_whitespace() // 1. Split the string
        .map(|i| {          // 2. Map applies code to every item
            // Parse the number part (slice from index 1 to end)
            // 'i[1..]' is roughly equivalent to python's 'i[1:]'
            let num: i32 = i[1..].parse().unwrap();

            // 3. The conditional logic
            if i.starts_with('R') {
                num
            } else {
                -num
            }
        })
        .collect();

    let mut pos = 50;
    let mut counter_part1 = 0;
    let mut counter_part2 = 0;

    for delta in deltas {
        // Determine direction: 1 if positive, -1 if negative
        let step = if delta > 0 { 1 } else { -1 };
        
        // Inner loop: iterate absolute value times
        // 0..n is the Rust equivalent of range(n)
        for _ in 0..delta.abs() {
            pos += step;

            // Bounds checking
            if pos == -1 { pos = 99; }
            if pos == 100 { pos = 0; }

            // Part 2 check (inside the movement)
            if pos == 0 {
                counter_part2 += 1;
            }
        }

        // Part 1 check (after the movement)
        if pos == 0 {
            counter_part1 += 1;
        }
    }

    println!("Day 1, part 1: {}", counter_part1);
    println!("Day 1, part 2: {}", counter_part2);
}